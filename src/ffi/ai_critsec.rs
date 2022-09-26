/* FIXME: We probably need to handroll this from the arnold api implementation
from ai_critsec.h

template<class Mutex>
class AtProfiledLockable {
public:
   void lock()
   {
      AiProfileBlock("thread blocked");
      mutex.lock();
   }
   bool try_lock()
   {
      AiProfileBlock("thread blocked");
      return mutex.try_lock();
   }
   void unlock() { mutex.unlock(); }

   using mutex_type         = Mutex;
   using native_handle_type = typename Mutex::native_handle_type;

   native_handle_type native_handle() { return mutex.native_handle(); }

   const mutex_type& get() const { return mutex; }
         mutex_type& get()       { return mutex; }

private:
   Mutex mutex;
};

--------

#[doc = " Wrapper for Lockable type mutexes, such as std::mutex and"]
#[doc = " std::recursive_mutex. Use this so that the arnold profiler can keep track of"]
#[doc = " of the time spent blocked waiting to lock. For instance, you can replace the following C++11/14 code:"]
#[doc = " \\code"]
#[doc = "std::mutex my_mutex;"]
#[doc = "std::lock_guard<std::mutex> my_guard(my_mutex);"]
#[doc = "\\endcode"]
#[doc = " with:"]
#[doc = " \\code"]
#[doc = "AtProfiledLockable<std::mutex> my_mutex;"]
#[doc = "std::lock_guard<AtProfiledLockable<std::mutex>> my_guard(my_mutex);"]
#[doc = "\\endcode"]
#[doc = ""]
#[doc = " As a convenience, the AtMutex and AtRecursiveMutex type aliases can be used"]
#[doc = " so that instead one would write:"]
#[doc = " \\code"]
#[doc = "AtMutex my_mutex;"]
#[doc = "std::lock_guard<AtMutex> my_guard(my_mutex);"]
#[doc = "\\endcode"]
#[doc = ""]
#[doc = " Note that starting with C++17 this can be further simplified to:"]
#[doc = " \\code"]
#[doc = "AtMutex my_mutex;"]
#[doc = "std::lock_guard my_guard(my_mutex);"]
#[doc = "\\endcode"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtProfiledLockable<Mutex> {
    pub mutex: Mutex,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<Mutex>>,
}
pub type AtProfiledLockable_mutex_type = Mutex;
pub type AtProfiledLockable_native_handle_type = [u8; 0usize];
#[doc = " Convenience type aliases. See \\ref AtProfiledLockable"]
pub type AtMutex = AtProfiledLockable<std_mutex>;
pub type AtRecursiveMutex = AtProfiledLockable<std_recursive_mutex>;
*/

#[doc = " Opaque data type for a critical section"]
pub type AtCritSec = *mut ::std::os::raw::c_void;
extern "C" {
    pub fn AiCritSecInit(cs: *mut AtCritSec);
}
extern "C" {
    pub fn AiCritSecInitRecursive(cs: *mut AtCritSec);
}
extern "C" {
    pub fn AiCritSecClose(cs: *mut AtCritSec);
}
extern "C" {
    pub fn AiCritSecEnter(cs: *mut AtCritSec);
}
extern "C" {
    pub fn AiCritSecLeave(cs: *mut AtCritSec);
}
