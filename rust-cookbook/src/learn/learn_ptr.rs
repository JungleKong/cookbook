use std::ptr;
use std::time::Instant;
/// # Safety 从原生指针构造vector
/// * `ptr` 必须与其类型正确对齐且非零。
/// * `ptr` 必须对 `T` 类型的 `elts` 连续元素的读取有效。
/// * 除非 `T: Copy`，否则在调用此函数后不得使用这些元素。
#[allow(dead_code)]
unsafe fn from_buf_raw<T>(ptr: *const T, elts: usize) -> Vec<T> {
    let mut dst = Vec::with_capacity(elts);

    // SAFETY: 我们的前提条件是确保源文件对齐和有效，而 `Vec::with_capacity` 确保我们有可用的空间来编写它们。
    ptr::copy(ptr, dst.as_mut_ptr(), elts);

    // SAFETY: 我们之前已经用这么大的容量创建了它，而以前的 `copy` 已经初始化了这些元素。
    dst.set_len(elts);
    dst
}



/// 将 `src` 的所有元素移到 `dst`，将 `src` 留空。
/// 手动实现append
#[allow(dead_code)]
fn append<T>(dst: &mut Vec<T>, src: &mut Vec<T>) {
    let src_len = src.len();
    let dst_len = dst.len();
    
    // 确保 `dst` 具有足够的容量来容纳所有 `src`。
    // 在原有长度的基础上增加了 `src` 的长度，而不是预留 src_len
    dst.reserve(src_len);
    
    unsafe {
        // 添加的调用总是安全的，因为 `Vec` 永远不会分配超过 `isize::MAX` 字节。
        let dst_ptr = dst.as_mut_ptr().add(dst_len);
        let src_ptr = src.as_ptr();
        
        // 截断 `src` 而不丢弃其内容。
        // 我们首先执行此操作，以避免在 panics 处出现问题时避免出现问题。
        src.set_len(0);
        
        // 这两个区域不能重叠，因为可变引用没有别名，并且两个不同的 vectors 不能拥有相同的内存。
        // 作用类似于memcpy，
        ptr::copy_nonoverlapping(src_ptr, dst_ptr, src_len);

        // 通知 `dst` 现在包含 `src` 的内容。
        dst.set_len(dst_len + src_len);
    }
}

#[cfg(test)]
mod test_learn {
    use super::*;

    #[test]
    fn test_from_buf_raw() {
        let v = vec![1, 2, 3, 4, 5];
        let ptr = v.as_ptr();
        let len = v.len();
        let v2 = unsafe { from_buf_raw(ptr, len) };
        assert_eq!(v, v2);
    }

    #[test]
    fn test_append() {
        let mut v1 = vec![1, 2, 3];
        let mut v2 = vec![4, 5, 6];
        append(&mut v1, &mut v2);
        assert_eq!(v1, vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(v2, vec![]);
    }

    #[test]
    fn test_drop_in_place() {
        use std::ptr;
        use std::rc::Rc;

        let last = Rc::new(1);
        let weak = Rc::downgrade(&last);

        let mut v = vec![Rc::new(0), last];

        unsafe {
            // 获取指向 `v` 中最后一个元素的裸指针。
            let ptr = &mut v[1] as *mut _;
            // 缩短 `v`，以防止丢弃最后一个项。
            // 我们首先这样做是为了防止 `drop_in_place` 低于 panics。
            v.set_len(1);
            // 如果没有调用 `drop_in_place`，则最后一个项将永远不会被删除，并且它管理的内存也会泄漏。
            ptr::drop_in_place(ptr);
        }

        assert_eq!(v, &[0.into()]);

        // 确保丢弃了最后一项。
        assert!(weak.upgrade().is_none());
    }
}