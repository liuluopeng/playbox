# 排序 




## 最大 和 次大
我每次冒出的朴素的想法: 先和次大的比较, 问题: 丢失次大的信息. 

正确的办法:  先和最大的比较. 


```rust
        let mut morest = 0;
        let mut more = 0;


        // if prev_res.res_throgh_me > more {
        //     more = prev_res.res_throgh_me;

        //     if more > morest {
        //         morest = more;

        //     }
        // }

        if prev_res.res_from_me > morest {
            more = morest; // 提前保存,
            morest = prev_res.res_from_me;
        } else if prev_res.res_from_me > more {
            more = prev_res.res_from_me;
        }


```