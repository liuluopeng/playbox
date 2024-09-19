

![[tp/README#主要新特性]]



### 安装view
`composer require topthink/think-view`





### 在终端里面打印信息
```php
use think\console\Output;

$msg = new Output();
$msg->write("hello");
```


### 数据库字符集
think PHP 6 默认字符集是utf8,改成`utf8mb4`后数据库可接受表情等字符。


