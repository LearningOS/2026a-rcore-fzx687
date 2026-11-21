# Lab3 实验 Report
## 实现的功能
1. `sys_get_time`: 基本直接复用之前的代码，只是获取 token 方式改变了。
2. `sys_mmap` 和 `sys_munmap`: 考虑到 lab2 的实现比较粗糙，这里重写了部分逻辑。在 **MemorySet** 中添加了新方法 `judge_conflict` 和 `remove_areas_with_start_and_end_vpn`。然后根据上面两个方法实现功能。
3. `sys_spawn`: 直接调用 `TaskControlBlock::new(elf_data)` 实现功能。
4. Stride 调度相关：在 **TaskControlBlockInner** 中添加 stride 和 priority 两个量。实现了 `get_task_with_stride` 方法用来代替调度中的 `fetcch`，然后正常实现算法。
5. `sys_set_priority`: 直接修改 priority。
## 问答题
1. 不是，因为 p2.stride + pass = 250 + 10 = 260（溢出） = 4 ，然后比较得出 p2.stride < p1.stride ，所以还是 p2 执行。
2. 类似归纳法的思路，初始条件满足。设第 k 步满足，此时 $STRIDE_{MAX}^{k} – STRIDE_{MIN}^{k} \le BigStride / 2$ （这里的上标代指第 k 步的 Stride）。对于第 $k + 1$ 步，$STRIDE_{new} \le STRIDE_{MIN}^{k} + BigStride / 2$，此时，$STRIDE_{MAX}^{k + 1} \le max\{STRIDE_{MAX}^{k},STRIDE_{MIN}^{k} + BigStride / 2\}$，$STRIDE_{MIN}^{k + 1} \ge STRIDE_{MIN}^{k}$ ，然后就能说明$STRIDE_{MAX}^{k + 1} – STRIDE_{MIN}^{k + 1} \le BigStride / 2$
3. 补全代码：
    ```
    use core::cmp::Ordering;

    struct Stride(u64);

    impl PartialOrd for Stride {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            let diff = (self.0 - other.0) as i64;
            if diff < 0 {
                return Some(Ordering::Greater);
            }else if diff > 0 {
                return Some(Ordering::Less);
            }else {
                return Some(Ordering::Equal);
            }
        }
    }

    impl PartialEq for Stride {
        fn eq(&self, other: &Self) -> bool {
            false
        }
    }
    ```
## 荣誉准则
1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：无
2. 此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：无
3.  我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。
4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。