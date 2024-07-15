# Minecraft Server Setup Script

このスクリプトは、特定のMinecraftバージョンとサーバータイプ（vanillaまたはforge）に基づいてMinecraftサーバーを自動的にセットアップします。また、Minecraftバージョンに応じて適切なJavaバージョンを自動的に選択してサーバーを起動します。

## 前提条件

- Ubuntu 22.04
- Java 8およびJava 17がインストールされていること
- `jq` コマンドがインストールされていること

### 必要なパッケージのインストール

```bash
sudo apt update
sudo apt install openjdk-8-jdk openjdk-17-jdk jq
```

## スクリプトの使用方法
### 1.スクリプトのダウンロードと実行権限の付与
```bash
git clone https://github.com/mmiura-2351/mc-server-templates.git
chmod +x create-server
```

### 2.サーバーの作成
以下のコマンドを使用して、サーバーを作成します。
```bash
./create-server [version] (vanilla|forge) [directory-name]
```

### 例
Minecraft 1.16.5 のForgeサーバーを forge_1.16.5_test というディレクトリに作成する場合:
```bash
./create-server 1.16.5 forge forge_1.16.5_test
```

### 3.サーバーの起動
作成されたディレクトリに移動し、start.sh スクリプトを実行してサーバーを起動します。
```bash
cd [directory-name]
./start.sh
```

> [!WARNING]
> 1.17以降のバージョンではstart.shでサーバーを起動できません。代わりに自動で生成されるrun.shを使用してください。

## スクリプトの詳細
1このスクリプトは以下のステップを実行します：
1. 指定されたディレクトリ名が既に存在するか確認し、存在する場合はエラーを表示して終了します。
2. 指定されたバージョンとサーバータイプに基づいて、適切なMinecraftサーバーまたはForgeサーバーをダウンロードします。
3. サーバーファイルを配置し、必要な設定ファイル（eula.txt）を作成します。
4. Minecraftのバージョンに応じて適切なJavaバージョンを選択し、サーバー起動用の start.sh スクリプトを作成します。

> [!WARNING]
> このスクリプトは個人使用のために作成したものであり、一切の責任を負いません。
> このREADME.mdはChatGPTで作成されたものです。
