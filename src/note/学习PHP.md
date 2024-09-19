
### 慎用 `var_dump()`。

>在一般的PHP页面中，我们使用var_dump来打印数据。是的，当我们使用TP5等框架的时候，我们会忘记我们使用过var_dump，这个时候，当我们使用框架再带的方法来打印数据的时候，就会出现多余的数据，这个时候我们就会抓瞎了。所以我们应该清除所有文件中的var_dump，换成框架的输出或者是日志输出更为妥当

### php 删除某键 :

```php
unset()
```

### json_encode() 格式化 中文字符
```php
json_encode($content, JSON_UNESCAPED_UNICODE|JSON_PRETTY_PRINT));
```

### 读写文件
```php
<?php
//	读取文件
$myfile = fopen("in.txt", "r") or die("Unable to open file!");
echo fread($myfile,filesize("in.txt"));
fclose($myfile);

//	写入文件
$myfile = fopen("out.txt", "w") or die("Unable to open file!");
fwrite($myfile,"hello");
fclose($myfile);

?>

```
### 获取文件内容
```php
file_get_contents()
```


### 数字格式化,补0
```php
// 0001
$numberFormat = str_pad("1", 4, "0", STR_PAD_LEFT);


str_pad(string,length,pad_string,pad_type)
//参数    描述
string      //必需。规定要填充的字符串。
length      //必需。规定新的字符串长度。如果该值小于字符串的原始长度，则不进行任何操作。
pad_string  //可选。规定供填充使用的字符串。默认是空白。
pad_type    //可选。规定填充字符串的哪边。
            //可能的值：
            STR_PAD_BOTH - //填充字符串的两侧。如果不是偶数，则右侧获得额外的填充。
            STR_PAD_LEFT - //填充字符串的左侧。
            STR_PAD_RIGHT - //填充字符串的右侧。默认。
```
