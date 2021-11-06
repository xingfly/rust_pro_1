# 作业
#### 列出3个常用的宏、3个常用的存储数据结构
常用宏：
* 功能模块宏：#[frame_support::pallet]
* 配置接口宏：#[pallet::config]
* 事件宏：#[pallet::event]
* 错误信息宏：#[pallet::error]
* 可调用函数宏：#[pallet::call]
* 权重宏(将值转换为交易费用，避免拒绝服务攻击)：#[pallet::weight(value)]

三个存储数据结构：
* 单值：StorageValue
* 映射：StorageMap
* 双键映射：StorageDoubleMap
#### 实现存证模块的功能，包括：创建存证；撤销存证。
0. 启动node_template
![启动程序](https://imgur.com/hC45Dtg.png)

1. 查询凭证0x00是否存在（不存在）
![不存在](https://imgur.com/q8NzRmN.png)
2. 创建一个0x00凭证
    ![创建凭证](https://imgur.com/PRGU47u.png)
3. 再次查询0x00凭证(95号区块时有了该凭证)
![再次查询](https://imgur.com/U1dy3rl.png)
4. 销毁0x00凭证
    ![销毁凭证](https://imgur.com/geEidCO.png)
    
5. 查询销毁是否成功(已经销毁)
    ![查询](https://imgur.com/He6YdxD.png)
#### 为存证模块添加新的功能，转移存证，接收两个参数，一个是内容的哈希值，另一个是存证的接收账户地址。
1. Alice创建一个凭证（和上面类似不截图了）
2. 将凭证转移给Bob
    ![转移](https://imgur.com/wtjf0YK.png)
3. Alice去销毁凭证
    ![Alice销毁](https://imgur.com/64352JL.png)
    由于没有权限销毁失败
    ![Alice销毁失败](https://imgur.com/XBcifjd.png)
4. Bob去销毁凭证
    由于凭证已经转移给Bob所以Bob可以销毁
    ![Bob销毁成功](https://imgur.com/3llolaz.png)