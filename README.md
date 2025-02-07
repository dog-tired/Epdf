# Epdf
PDF editor written in rust, under development


<center>
    <img style="border-radius: 5px;"
         src="readmeFile/image.png" 
         alt="baymax"
         width="70%" ><br/>
    <span>软件使用</span>
</center>

- --help: 显示帮助信息
- --dis 显示pdf ：
    - .\Epdf.exe --dis concat-test.pdf
- --copy 新建pdf ：
    - .\Epdf.exe --copy concat-test.pdf 1-2,2,2,2,2,1,1,1
- --water 加水印 : 
    - .\Epdf.exe --water concat-test.pdf waterMark
- --2images 转图片: 
    - .\Epdf.exe --2images concat-test.pdf



# Epdf

Epdf 是一款使用 Rust 编写的 PDF 编辑器，目前仍在开发中。它提供了一系列实用的命令行选项，方便用户对 PDF 文件进行操作。

## 命令行选项

### --help
显示帮助信息，帮助用户快速了解软件的使用方法。

### --dis 显示pdf
通过该选项，用户可以指定要显示的 PDF 文件。例如：
```sh
.\Epdf.exe --dis concat-test.pdf
```

### --copy 新建pdf
此选项用于新建一个 PDF 文件，你需要指定源 PDF 文件以及相关的页面选择参数。例如：
```sh
.\Epdf.exe --copy concat-test.pdf 1-2,2,2,2,2,1,1,1
```
上述命令中的 `1-2,2,2,2,2,1,1,1` 为页面选择参数，具体含义可根据软件的实际逻辑确定。

### --water 加水印
使用该选项为指定的 PDF 文件添加水印。示例如下：
```sh
.\Epdf.exe --water concat-test.pdf waterMark
```
其中 `waterMark` 为要添加的水印内容。

### --2images 转图片
该选项可将指定的 PDF 文件转换为图片。使用示例：
```sh
.\Epdf.exe --2images concat-test.pdf
```

请注意，由于该软件仍在开发中，部分功能可能存在不稳定或不完善的情况。在使用过程中如有任何问题或建议，欢迎反馈给开发者。 