
# 合约部署
## erc20
![image](https://github.com/xusanduo08/erc20/assets/17930163/39e9ecfe-ce21-4f43-85f1-e310e7300075)

## erc721
![image](https://github.com/xusanduo08/erc20/assets/17930163/f2c3af71-8ab4-4b9e-bfd6-bb841ef9ab1a)

## markets
![image](https://github.com/xusanduo08/erc20/assets/17930163/90b1a522-1f1e-4911-8ac3-ab5f3d8861c2)

# 合约间交互

## 授权markets可以调用alice账户资金
1. 设置参数
![image](https://github.com/xusanduo08/erc20/assets/17930163/2a03248f-565e-47be-9aa0-67f4a18d45b0)

2. 提交
![image](https://github.com/xusanduo08/erc20/assets/17930163/2917db25-c1bf-4670-831c-7f57bb998e22)

## 调用market 铸造erc721

～～这块出错了，在erc20里approve了markets，但是依然没法dry run，待解决～～ ---- 本地节点部署错了
<img width="1455" alt="image" src="https://github.com/xusanduo08/erc20/assets/17930163/58314128-d4be-432e-a444-96d0718963eb">

在erc721中查询alice下的nft，可以看到数量是1
<img width="1441" alt="image" src="https://github.com/xusanduo08/erc20/assets/17930163/405af26f-3a7f-4f10-8de5-aaaa8b6ee9c4">


