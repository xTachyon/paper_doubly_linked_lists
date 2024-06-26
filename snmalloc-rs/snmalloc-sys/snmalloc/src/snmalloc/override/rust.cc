#define SNMALLOC_NAME_MANGLE(a) sn_##a
#include "malloc.cc"

#include <cstring>

#ifndef SNMALLOC_EXPORT
#  define SNMALLOC_EXPORT
#endif

using namespace snmalloc;

extern "C" SNMALLOC_EXPORT void*
SNMALLOC_NAME_MANGLE(rust_alloc)(size_t alignment, size_t size)
{
  return ThreadAlloc::get().alloc(aligned_size(alignment, size));
}

extern "C" SNMALLOC_EXPORT void*
SNMALLOC_NAME_MANGLE(rust_alloc_zeroed)(size_t alignment, size_t size)
{
  return ThreadAlloc::get().alloc<YesZero>(aligned_size(alignment, size));
}

extern "C" SNMALLOC_EXPORT void
SNMALLOC_NAME_MANGLE(rust_dealloc)(void* ptr, size_t alignment, size_t size)
{
  ThreadAlloc::get().dealloc(ptr, aligned_size(alignment, size));
}

extern "C" SNMALLOC_EXPORT void* SNMALLOC_NAME_MANGLE(rust_realloc)(
  void* ptr, size_t alignment, size_t old_size, size_t new_size)
{
  size_t aligned_old_size = aligned_size(alignment, old_size),
         aligned_new_size = aligned_size(alignment, new_size);
  if (
    size_to_sizeclass_full(aligned_old_size).raw() ==
    size_to_sizeclass_full(aligned_new_size).raw())
    return ptr;
  void* p = ThreadAlloc::get().alloc(aligned_new_size);
  if (p)
  {
    std::memcpy(p, ptr, old_size < new_size ? old_size : new_size);
    ThreadAlloc::get().dealloc(ptr, aligned_old_size);
  }
  return p;
}

extern "C" SNMALLOC_EXPORT void SNMALLOC_NAME_MANGLE(rust_statistics)(
  size_t* current_memory_usage, size_t* peak_memory_usage)
{
  *current_memory_usage = StandardConfig::Backend::get_current_usage();
  *peak_memory_usage = StandardConfig::Backend::get_peak_usage();
}

void* operator new(size_t size)
{
  auto ptr = malloc(size);
  if (ptr == nullptr)
  {
    fprintf(stderr, "snmalloc: allocation failed\n");
    abort();
  }
  return ptr;
}

void operator delete(void* ptr)
{
  free(ptr);
}
void operator delete(void* ptr, size_t)
{
  free(ptr);
}

extern "C" SNMALLOC_EXPORT Alloc* SNMALLOC_NAME_MANGLE(rust_inst_create)()
{
  return new Alloc;
}

extern "C" SNMALLOC_EXPORT void
SNMALLOC_NAME_MANGLE(rust_inst_destroy)(Alloc* alloc)
{
  delete alloc;
}

extern "C" SNMALLOC_EXPORT void* SNMALLOC_NAME_MANGLE(rust_inst_alloc)(
  Alloc* alloc, size_t alignment, size_t size)
{
  return alloc->alloc(aligned_size(alignment, size));
}

extern "C" SNMALLOC_EXPORT void* SNMALLOC_NAME_MANGLE(rust_inst_alloc_zeroed)(
  Alloc* alloc, size_t alignment, size_t size)
{
  return alloc->alloc<YesZero>(aligned_size(alignment, size));
}

extern "C" SNMALLOC_EXPORT void SNMALLOC_NAME_MANGLE(rust_inst_dealloc)(
  Alloc* alloc, void* ptr, size_t alignment, size_t size)
{
  alloc->dealloc(ptr, aligned_size(alignment, size));
}