# rust


## 构建相关. cargo相关. 

我的一个文件夹下面有很多rust项目, 如何快速clean这些项目的target. 我不想一个一个进去执行cargo clean?
```bash
#!/bin/bash

# 遍历当前目录下的所有子目录
for dir in */ ; do
    # 检查是否存在`Cargo.toml`文件，这通常表示一个Rust项目
    if [ -f "$dir/Cargo.toml" ]; then
        # 进入项目目录
        cd "$dir"
        # 执行cargo clean命令
        cargo clean
        # 返回到原始目录
        cd ..
    fi
done

echo "所有Rust项目的target目录已清理完成。"
```


## 自定义排序.


```
arr是Vec<Vec<i32>>类型. 请你排序, 依据是: arr[i][0] < arr[j][0] 如果相同, 那么 arr[i][1] > arr[j][1].
```

```rust
    // 定义比较函数
    arr.sort_by(|a, b| {
        if a[0] == b[0] {
            b[1].cmp(&a[1])
        } else {
            a[0].cmp(&b[0])
        }
    });

```

还有 
```rust
    let mut vec = vec![vec![1, 2], vec![3, 4]];

    // 按照子数组的第二个元素从小到大排序
    vec.sort_by_key(|sub_vec| sub_vec[1]);
```

```rust
    let mut vec = vec![vec![1, 2], vec![3, 4]];

    // 按照子数组的第二个元素从小到大排序
    vec.sort_by(|a, b| a[1].cmp(&b[1]));
```


## 智能指针

`take()`

```
core::option::Option
pub const fn take(&mut self) -> Option<T>
Takes the value out of the option, leaving a None in its place.
```


take()的作用: 
可以删除链表元素:

```rust


pub fn delete_even_nodes(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut current = &mut head;

    while let Some(ref mut node) = current {
        if node.val % 2 == 0 {
            *current = node.next.take();
        } else {
            current = &mut node.next;
        }
    }

    head
}
```

```
这行代码是删除偶数节点的关键操作。我们来详细解释一下：

node.next.take()：take()方法会将node.next的值取出，并将node.next置为None。这样，node.next的所有权被转移到了我们手中。
*current = node.next.take();：current是指向链表中一个Option的可变引用。通过*current进行解引用，我们可以修改它的值。将current指向node.next的值（即node.next.take()的结果）。这相当于跳过当前节点，直接指向下一个节点，从而在逻辑上删除了当前节点。
```


```rust


pub fn delete_even_nodes(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut current = &mut head;

    while let Some(ref mut node) = current {
        if node.val % 2 == 0 {
            // Skip the current node by linking to the next node
            *current = node.next.as_mut().map(|next_node| next_node.clone());
        } else {
            current = &mut node.next;
        }
    }

    head
}
```


`as_ref()`
```
core::option::Option
pub const fn as_ref(&self) -> Option<&T>
Converts from &Option<T> to Option<&T>.
```


`as_mut()`
```
core::option::Option
pub const fn as_mut(&mut self) -> Option<&mut T>
Converts from &mut Option<T> to Option<&mut T>.
```




辨析 ```

`Rc`

```
alloc::rc::Rc
fn clone(&self) -> Self
Makes a clone of the Rc pointer.

This creates another pointer to the same allocation, increasing the strong reference count.
```

`Rc::clone`的一个例子:

```rust
let left = node.left.as_ref();
let left_node: Option<Rc<RefCell<TreeNode>>> = match left {
	None => None,
	Some(left) => Some(Rc::clone(left)),
};
```


``` rust 
left = node.left.as_ref().map(Rc::Clone)
```



`RefCell`
```
core::cell::RefCell
pub fn borrow(&self) -> Ref<'_, T>
Immutably borrows the wrapped value.

The borrow lasts until the returned Ref exits scope. Multiple immutable borrows can be taken out at the same time.
```
`borrow()`




`borrow_mut()`
```
core::cell::RefCell
pub fn borrow_mut(&self) -> RefMut<'_, T>
Mutably borrows the wrapped value.

The borrow lasts until the returned RefMut or all RefMuts derived from it exit scope. The value cannot be borrowed while this borrow is active.
```




智能指针不能解决的时候: 
或者不想使用智能指针的时候: 
`unsafe`

```rust
    pub fn delete_even_nodes(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut current: *mut Option<Box<ListNode>> = &mut head;

        unsafe {
            while let Some(ref mut node) = *current {
                if node.val % 2 == 0 {
                    // Move the next node to the current position
                    *current = node.next.take();
                } else {
                    // Move to the next node
                    current = &mut node.next as *mut _;
                }
            }
        }

        head
    }
```



## tip 


## 什么时候用 node.as_ref()   


`let mut prev_node: Option<&Box<ListNode>> = dummy_head.as_ref();`
    `let mut prev_node: &mut Option<Box<ListNode>> = &mut  dummy_head;`  这两种办法的优劣是什么?


GPT:
```


这两种办法在处理Rust中链表节点的引用和可变引用时各有优劣，具体如下：
第一种方法
let mut prev_node: Option<&Box<ListNode>> = dummy_head.as_ref();
优点：
安全性：使用不可变引用（&），确保在操作过程中不会意外修改链表中的数据。适用于只读操作。
简单性：代码逻辑简单，适合需要遍历或读取链表节点的场景。
缺点：
不可变性：无法修改链表中的节点。如果需要对链表进行修改（如插入或删除节点），这种方法不适用。
灵活性：局限于只读操作，灵活性较低。

第二种方法
let mut prev_node: &mut Option<Box<ListNode>> = &mut dummy_head;
优点：
可变性：使用可变引用（&mut），允许修改链表中的节点，非常适合需要对链表进行增删改操作的场景。
灵活性：可以进行更复杂的操作，如插入、删除节点等。
缺点：
复杂性：代码逻辑相对复杂，需要小心处理可变引用，避免数据竞争和借用检查错误。
安全性：由于允许修改链表，可能会引入一些潜在的错误，需要更加谨慎地处理。
总结
第一种方法适用于只读操作，优点是简单、安全，但缺点是无法修改链表。
第二种方法适用于需要修改链表的场景，优点是灵活、可变，但缺点是代码复杂性增加，需要注意处理可变引用的安全性。
具体选择哪种方法，取决于你对链表的操作需求。如果只是遍历或读取链表节点，第一种方法更合适；如果需要修改链表结构，第二种方法则是必需的。





```


##  如何从 `&mut     Option<Rc<RefCell<TreeNode>>>`得到 `Option<Rc<RefCell<TreeNode>>>`类型?

```

在 Rust 中，&mut Option<Rc<RefCell<TreeNode>>> 是一个可变引用，指向一个 Option<Rc<RefCell<TreeNode>>> 类型的值。如果你想从这个可变引用中得到一个 Option<Rc<RefCell<TreeNode>>> 类型的值，你有几种方法可以做到这一点，具体取决于你希望如何处理这个值。


克隆值：如果你希望得到一个新的 Option<Rc<RefCell<TreeNode>>>，并且不修改原来的值，可以使用 clone 方法。需要注意的是，这要求 Rc<RefCell<TreeNode>> 实现了 Clone 特性。

use std::rc::Rc;
use std::cell::RefCell;

struct TreeNode {
    // TreeNode 的字段
}

fn get_option_clone(opt: &mut Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    opt.clone()
}



提取和替换：如果你希望从 &mut Option<Rc<RefCell<TreeNode>>> 中提取值，并且将原来的值替换为 None，可以使用 Option::take 方法。
use std::rc::Rc;
use std::cell::RefCell;
struct TreeNode {
    // TreeNode 的字段
}
fn take_option(opt: &mut Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    opt.take()
}



借用值：如果你只需要临时借用 Option<Rc<RefCell<TreeNode>>> 的值，而不需要所有权，可以直接解引用并使用该引用。
use std::rc::Rc;
use std::cell::RefCell;
struct TreeNode {
    // TreeNode 的字段
}
fn borrow_option(opt: &mut Option<Rc<RefCell<TreeNode>>>) -> Option<&Rc<RefCell<TreeNode>>> {
    opt.as_ref()
}


修改值：如果你需要修改 Option<Rc<RefCell<TreeNode>>> 中的值，可以直接通过解引用来访问和修改。
use std::rc::Rc;
use std::cell::RefCell;
struct TreeNode {
    // TreeNode 的字段
}
fn modify_option(opt: &mut Option<Rc<RefCell<TreeNode>>>) {
    if let Some(ref mut node) = opt {
        // 修改 node
    }
}
选择哪种方法取决于你的具体需求：是否需要克隆值、是否需要提取值并替换原来的值、是否只需要借用值，或者是否需要修改值。

```



## 模式匹配是否需要添加ref mut
```
什么时候需要ref mut node 什么时候不需要?

在 Rust 中，ref mut 用于创建一个可变引用，以便在模式匹配中对值进行修改。具体来说，ref mut 允许我们在模式匹配的同时获取一个可变引用，从而能够修改匹配到的值。

在 delete_even_nodes 函数中，使用 ref mut 是为了能够在 while let 循环中修改当前节点的值和链表结构。让我们详细解释一下什么时候需要 ref mut，什么时候不需要。

需要 ref mut 的情况
在以下情况下需要使用 ref mut：

修改当前节点的值或结构：如果我们需要在模式匹配中修改当前节点的值或其内部结构（如 next 指针），我们需要一个可变引用。
例如，在 while let Some(ref mut node) = current 这行代码中，我们需要 ref mut 是因为我们可能会修改 node.next，以跳过偶数节点：

while let Some(ref mut node) = current {
    if node.val % 2 == 0 {
        // Skip the current node by linking to the next node
        if let Some(ref mut next_node) = node.next {
            *current = Some(next_node.clone());
        } else {
            *current = None;
        }
    } else {
        current = &mut node.next;
    }
}
在这个代码段中，ref mut node 允许我们在匹配到 Some(node) 后，仍然可以修改 node 的内容。

不需要 ref mut 的情况
在以下情况下不需要使用 ref mut：

仅仅是借用而不修改：如果我们只需要读取值而不进行修改，或者我们不需要可变引用来进行操作，那么可以直接使用不可变引用或直接解构。
例如，如果我们只需要读取节点的值而不修改它，可以直接使用 ref 或不使用 ref：

if let Some(node) = current {
    // 只需要读取 node.val
    if node.val % 2 == 0 {
        // 处理偶数节点
    } else {
        // 处理非偶数节点
    }
}
在这种情况下，我们只是读取 node.val，并不需要修改 node，因此不需要 ref mut。

具体示例
让我们通过一个具体的例子来对比 ref mut 和不使用 ref mut 的情况：

// 使用 ref mut
while let Some(ref mut node) = current {
    if node.val % 2 == 0 {
        // 需要修改 node.next
        if let Some(ref mut next_node) = node.next {
            *current = Some(next_node.clone());
        } else {
            *current = None;
        }
    } else {
        current = &mut node.next;
    }
}

// 不使用 ref mut
while let Some(node) = current {
    if node.val % 2 == 0 {
        // 只读取 node.val，不修改 node
        if let Some(next_node) = node.next.as_ref() {
            *current = Some(next_node.clone());
        } else {
            *current = None;
        }
    } else {
        current = &mut node.next;
    }
}
在第一段代码中，我们使用 ref mut 是因为我们需要修改 node.next。在第二段代码中，我们不需要 ref mut，因为我们只是读取 node.val，并不修改 node。

总结来说，是否需要 ref mut 取决于我们是否需要在模式匹配中对值进行修改。如果需要修改，则使用 ref mut；如果只需要读取，则可以不使用 ref mut。
```




## 删除节点
LCR 136. 删除链表的节点 - 力扣（LeetCode） 
https://leetcode.cn/problems/shan-chu-lian-biao-de-jie-dian-lcof/solutions/718093/in-rust-we-trust-list-delete_node-by-zha-gexo/ 

```rust
    pub fn delete_node(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut root = head;
        let mut head = &mut root;
        while let Some(node) = head {
            if node.val == val {
                *head = node.next.take();
                break;
            }
            head = &mut head.as_mut().unwrap().next;
        }
        root
    }

作者：Tab Liu
链接：https://leetcode.cn/problems/shan-chu-lian-biao-de-jie-dian-lcof/solutions/1166701/jian-zhi-offer-18-shan-chu-lian-biao-de-gy7mg/
来源：力扣（LeetCode）
著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。
```

```rust
    pub fn delete_node(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut head = Some(Box::new(ListNode { val: 1, next: head }));
        let mut root = &mut head;
        while let Some(node) = root {
            let next_node = &mut node.next;
            if let Some(t) = next_node {
                if t.val == val {
                    node.next = t.next.take();
                }
            }
            root = &mut node.next;
        }
        head.unwrap().next
    }

作者：zhangchunzhong
链接：https://leetcode.cn/problems/shan-chu-lian-biao-de-jie-dian-lcof/solutions/718093/in-rust-we-trust-list-delete_node-by-zha-gexo/
来源：力扣（LeetCode）
著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。
```

```rust
    pub fn delete_node(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut head = Some(Box::new(ListNode {
            next: head,
            val: 0
        }));
        let mut p = &mut head;

        while p.as_ref().unwrap().next != None {
            if p.as_ref().unwrap().next.as_ref().unwrap().val == val {
                p.as_mut().unwrap().next = p.as_mut().unwrap().next.take().unwrap().next;
                break;
            }
            p = &mut p.as_mut().unwrap().next;
        }

        head.unwrap().next
    }

作者：icespark
链接：https://leetcode.cn/problems/shan-chu-lian-biao-de-jie-dian-lcof/solutions/170029/rust-fang-fa-qi-shi-ye-mei-zen-yao-ma-fan-by-icesp/
来源：力扣（LeetCode）
著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。
```