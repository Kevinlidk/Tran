-   添加开机自启动教程
-   完善许可证的使用
-   改进版本获取 api，通过远端进行检测
    -   CI 将 版本号推送至 Cloudflare KV
    -   worker 直接返回版本号
-   直接匹配字符串版本号，取消请求
