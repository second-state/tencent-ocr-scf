[English](README-en.md) | [Live demo!](https://sls-website-ap-beijing-imubol-1302315972.cos-website.ap-beijing.myqcloud.com/)

# 说明
目前这个演示性质的 demo 基于 Tesseract 构建的，没有针对中文进行优化，精确度有点低。但是和 OCR SaaS 不同的是，这个 OCR 服务使用 serverless 架构，非常灵活，开发者可以根据自己的应用场景训练Tesseract 字库，提高图片的识别效果。比如通用类文字识别、身份证等证件类识别、票据类文字识别等场景。

使用 serverless 的好处之二：开发者不用关注底层架构，只需关注业务逻辑就行了。好处之三：开发者只需按使用量付费，再加上腾讯云 serverless 每个月都会有一定的免费调用额度，开发自己的 demo 项目基本不用付费。

公开教程：
* [超级详细的Tesseract-OCR样本训练方法](https://blog.csdn.net/sylsjane/article/details/83751297)
* [通过 tesseract + OCR 识别身份证（windows 版）](https://blog.csdn.net/xingfeichen/article/details/69944212)
* [基于tesseract4.0 + opencv + Python 的身份证信息识别（主讲原理，无源码）](https://blog.csdn.net/baidu_33473810/article/details/85320365)
* [Tesseract OCR iOS(源码)](https://github.com/mobyIsMe/Tesseract-OCR)
* [Tesseract-OCR的简单使用与训练](https://www.cnblogs.com/cnlian/p/5765871.html)
* [熟肉视频：使用YOLOv4，OpenCV, Tesseract OCR实现车牌识别](https://www.bilibili.com/video/BV1jy4y1q7rm?from=search&seid=10328151300948832917)

# 快速开始

首先请确认已经安装了 [Serverless Framework](https://www.serverless.com/framework/docs/providers/tencent/guide/installation/) . Clone 这个 repo，然后用下面的命令部署整个云原生应用！

```
$ sls deploy
... ...
  website:       https://sls-website-ap-hongkong-kfdilz-1302315972.cos-website.ap-hongkong.myqcloud.com
  vendorMessage: null

63s › tencent-ocr-scf › "deploy" ran for 3 apps successfully.
```

在浏览器中加载网站 URL，就开始使用函数来识别照片中的对象啦。

# 创建你自己的文本识别云函数

Fork 这个 repo，使用 `Code | Open with Codespaces` 按钮来在浏览器中打开 Github Codespaces IDE 。第一次启动时，需要花费几分钟。 

## 低代码开发

一旦 Codespaces IDE 启动了, 你就可以根据自己的应用程序需求来对源代码进行修改，自定义函数。

* 要识别另一种语言，把该语言的 [traineddata file](https://github.com/tesseract-ocr/tessdata) 加入到 `scf/` 目录下.
* 在 `src/main.rs` 文件中更改语言设置以及数据预处理和后处理逻辑。
* 在 `website/content/index.html` 文件中对前端UI进行更改。

## 创建

在 Codespaces 打开 `Terminal` 窗口, 然后运行下面的命令行来创建云函数。

```
$ ssvmup build --enable-aot
```

## 部署

在 `Terminal` 窗口，运行下面的命令行将 Tesseract OCR 云函数部署到腾讯云上。

```
$ cp pkg/scf.so scf/

$ sls deploy
... ...
  website:       https://sls-website-ap-hongkong-kfdilz-1302315972.cos-website.ap-hongkong.myqcloud.com
```

在浏览器内加载部署好的 URL。 Have fun!

# 在本地机器上部署

如果你不能或不想使用 Github Codespaces，那可以在自己的计算机（或Docker映像）上安装 ssvmup 和 serverless framework 工具链来构建和部署文本识别 serverless 函数。
[安装 ssvmup 工具](https://www.secondstate.io/articles/ssvmup/)

通过 NPM 安装 Serverless Framework。

```
$ npm install -g serverless
```

准备工作已经做完了，现在你可以参照上文提到的 Codespaces 的创建和部署教程来创建自己的云函数。

