

# rust热加载的办法  auto reloading  ： 
需要安装：
```shell
cargo install cargo-watch
```
运行： 
```
 cargo watch -x run
```
对于需要选择的bin： 
```
cargo watch -x "run --bin helloworld-server"
```
# go 根据proto生成代码： 

```
 protoc --go_out=. --go_opt=paths=source_relative \  
    --go-grpc_out=. --go-grpc_opt=paths=source_relative \
    proto/helloworld.proto

```


交叉编译的时候报错，说cc有问题。 
得知mac os用的是clang而不是gcc。
在windows下利用wsl编译linux， 但是sqlx在编译的时候报错说数据库访问拒绝：
这说明sqlx在编译的时候会试着去连接数据库。
我觉得在wsl安装mysql很麻烦，会有宿主网络和wsl内网的问题。
2023年12月03日把小服务器清空后安装了宝塔， 数据库非常慢。
然后安装了rust， 但是编译报错。
后来反应过来， 删去了mac复制过去的`.cargo/config`


这个grpc的作用是电表从低到高 展示。




在大服务器安装，报错：没有protoc。
yum安装protoc， 但是版本是2.5的， 编译报错。
在小服务器上编译安装，然后在大服务器上运行：

```
build.rs  Cargo.lock  Cargo.toml  helloworld-server  proto  src  target
[root@VM-4-6-centos tonic]# ./helloworld-server 
./helloworld-server: /lib64/libm.so.6: version `GLIBC_2.29' not found (required by ./helloworld-server)
./helloworld-server: /lib64/libc.so.6: version `GLIBC_2.33' not found (required by ./helloworld-server)
./helloworld-server: /lib64/libc.so.6: version `GLIBC_2.28' not found (required by ./helloworld-server)
./helloworld-server: /lib64/libc.so.6: version `GLIBC_2.32' not found (required by ./helloworld-server)
./helloworld-server: /lib64/libc.so.6: version `GLIBC_2.18' not found (required by ./helloworld-server)
./helloworld-server: /lib64/libc.so.6: version `GLIBC_2.25' not found (required by ./helloworld-server)
./helloworld-server: /lib64/libc.so.6: version `GLIBC_2.34' not found (required by ./helloworld-server)
```

# 软件版本升级

## 来自你的消息:
cent os用yum安装的protoc是2.5版本，如何升级到3版本？

## 来自 ChatGPT 的消息:
在CentOS上，使用yum安装的软件包通常是通过官方软件仓库提供的。然而，官方软件仓库中的软件版本可能会滞后于最新版本。如果你想升级到较新的版本，可能需要使用其他方法。

要升级到Protocol Buffers（protobuf）3版本，你可以按照以下步骤进行操作：

1. 确认你已经安装了必要的依赖项：
   ```
   sudo yum install autoconf automake libtool curl make gcc-c++ unzip
   ```

2. 下载Protocol Buffers的源代码：
   ```
   wget https://github.com/protocolbuffers/protobuf/releases/download/v3.18.1/protobuf-all-3.18.1.tar.gz
   ```

3. 解压源代码包：
   ```
   tar -xzvf protobuf-all-3.18.1.tar.gz
   ```

4. 进入解压后的目录：
   ```
   cd protobuf-3.18.1
   ```

5. 编译和安装Protocol Buffers：
   ```
   ./configure
   make
   sudo make install
   ```

6. 确认安装是否成功：
   ```
   protoc --version
   ```

   如果输出显示为3.x.x的版本号，说明升级成功。

请注意，这种方法会从源代码编译和安装Protocol Buffers，可能需要一些时间和额外的依赖项。另外，由于系统中可能已经存在旧版本的protoc，你可能需要使用`sudo make install`命令来覆盖系统中的旧版本。

如果你希望更细粒度地控制软件版本，你还可以考虑使用包管理工具如`apt`或者`dnf`来安装Protocol Buffers。这些工具可能会提供更新的软件版本。

## 来自你的消息:
protoc: error while loading shared libraries: libprotobuf.so.29: cannot open shared object file: No such file or directory

## 来自 ChatGPT 的消息:
这个错误通常是由于系统无法找到所需的共享库文件引起的。在升级Protocol Buffers后，系统中的链接可能会指向旧版本的共享库文件，导致无法找到新版本的库文件。

为了解决这个问题，你可以尝试以下方法：

1. 更新动态链接库缓存：
   ```
   sudo ldconfig
   ```

   这将更新系统中的动态链接库缓存，使系统能够找到新安装的库文件。

2. 如果更新动态链接库缓存后仍然出现问题，可以尝试创建一个软链接来解决共享库的问题。首先，找到新版本的libprotobuf.so文件的路径。可以使用以下命令来查找：
   ```
   sudo find / -name libprotobuf.so*
   ```

   找到路径后，创建一个软链接到该文件：
   ```
   sudo ln -s /path/to/libprotobuf.so.29 /usr/lib/libprotobuf.so.29
   ```

   将`/path/to/libprotobuf.so.29`替换为实际路径。

3. 如果上述方法仍然无法解决问题，可能需要检查系统的库路径配置。确保`/usr/lib`或者其他包含库文件的路径在系统的`LD_LIBRARY_PATH`环境变量中。可以使用以下命令来查看环境变量的值：
   ```
   echo $LD_LIBRARY_PATH
   ```

   如果没有包含所需的路径，可以通过编辑`/etc/ld.so.conf`文件或者创建一个新的`.conf`文件来添加路径。然后运行以下命令来更新配置：
   ```
   sudo ldconfig
   ```

   请注意，对于不同的系统配置和版本，路径和配置可能会有所不同。

通过上述步骤，你应该能够解决找不到共享库文件的问题，并成功运行升级后的Protocol Buffers。如果问题仍然存在，建议查看系统日志以获取更多详细的错误信息，或者尝试在开发者社区或论坛上寻求帮助。



# mac os 安装grpc
```
brew install grpc 
brew install protobuf 
brew install protoc-gen-go
brew install protoc-gen-go-grpc
```

# 部署的顺序：
因为我现在gin里面， rpc一中断服务， gin就崩溃了。
- 编译rust。
- 上传gin的交叉编译的文件。
- 重启gin服务。

**清除 Git 缓存**：
运行以下命令来清除 Git 缓存，以便使新的排除项生效：
```
git rm -r --cached .

```
```
git commit -m "更新 .gitignore 文件以生效新的排除项"
```



日期差8小时的问题：
[Rust 练习] chrono 时间常用操作汇总 (持续补充...) - 知乎 
https://zhuanlan.zhihu.com/p/622979623 

2023年12月04日 
发现 电表更新时间是几个小时之前的， 青窝云的定时任务是1小时更新一次电表。    而updated_at这个字段和电表更新时间的字段时间差几个小时。
清空eb code表试试。



# 如何清除密码、 log 在git中 
```
如果你不小心将敏感信息（如密码配置）上传到 Git 仓库，并且希望忽略、删除该配置文件，并使其在历史提交中不可见，你可以按照以下步骤进行操作：

1. 首先，确保你已经备份了重要的配置文件和 Git 仓库。
    
2. 删除配置文件：从 Git 仓库中彻底删除该配置文件，包括所有历史提交中的版本。你可以使用以下命令来删除文件：
    
    `git filter-branch --force --index-filter 'git rm --cached --ignore-unmatch <config 文件路径>' --prune-empty --tag-name-filter cat -- --all`
    
    请将`<config 文件路径>`替换为实际的配置文件路径。这个命令将会遍历所有历史提交，并删除包含指定配置文件的提交。
    
    注意：这个命令会修改 Git 仓库的历史记录，因此在执行之前请确保你已经了解并理解了其影响。
    
3. 强制推送到远程仓库：由于 Git 仓库的历史记录已经被修改，你需要使用强制推送命令来更新远程仓库。执行以下命令：
    
    `git push origin --force --all`
    
    这会将本地仓库的修改强制推送到远程仓库，包括历史记录的修改。
    

请注意，这些操作将会修改 Git 仓库的历史记录，因此在执行之前请确保你已经了解并理解了其影响。另外，其他人可能已经克隆了你的仓库，所以你需要与他们协商，并确保他们也更新了仓库，以避免出现不一致的情况。
```