
- [ ]  用postman实现登录。

验证码  返回了一张图片，还有cookie
https://www.jianshu.com/p/ff57f1e8c054


打开隐私窗口，看看登录的时候的header：
```json
{
    "host": "localhost:9090",
    "connection": "keep-alive",
    "content-length": "126",
    "sec-ch-ua": "\"Chromium\";v=\"106\", \"Microsoft Edge\";v=\"106\", \"Not;A=Brand\";v=\"99\"",
    "accept": "application\/json, text\/plain, *\/*",
    "content-type": "application\/json;charset=UTF-8",
    "sec-ch-ua-mobile": "?0",
    "user-agent": "Mozilla\/5.0 (X11; Linux x86_64) AppleWebKit\/537.36 (KHTML, like Gecko) Chrome\/106.0.0.0 Safari\/537.36 Edg\/106.0.1370.52",
    "sec-ch-ua-platform": "\"Linux\"",
    "origin": "http:\/\/localhost:9090",
    "sec-fetch-site": "same-origin",
    "sec-fetch-mode": "cors",
    "sec-fetch-dest": "empty",
    "referer": "http:\/\/localhost:9090\/admin\/login?redirect=%2Fadmin%2Fhome",
    "accept-encoding": "gzip, deflate, br",
    "accept-language": "zh-CN,zh;q=0.9",
    "cookie": "cb_lang=zh-cn; WS_ADMIN_URL=ws:\/\/localhost:9090\/notice; WS_CHAT_URL=ws:\/\/localhost:9090\/msg; PHPSESSID=86653f799777ff19f3989b145630b3c7; collapsed=true"
}

```

postman的header：
```json
{
    "user-agent": "ApiPOST Runtime +https:\/\/www.apipost.cn",
    "content-type": "application\/json",
    "accept": "*\/*",
    "accept-encoding": "gzip, deflate, br",
    "connection": "keep-alive",
    "phpsessid": "96dd5358fa126da11c9c59455f654ae2",
    "host": "192.168.2.112:9090",
    "content-length": "51"
}

```

尝试：自定义cookie



登录： postman发送 导致了 php中止运行
```php
[$account, $password, $imgcode, $key, $captchaVerification, $captchaType] = $this->request->postMore([  
    'account',  
    'pwd',  
    ['imgcode', ''],  
    ['key', ''],  
    ['captchaVerification', ''],  
    ['captchaType', '']  
], true);

Array
(
    [0] => admin
    [1] => 123456
    [2] => msah
    [3] => 635cc6f4c835c
    [4] => 
    [5] => blockPuzzle
)

```



