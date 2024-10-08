# About  
このツールはMinecraftのサーバーをセットアップするためのものです。  

## 開発環境  
- Ubuntu 22.04
- Java 8およびJava 17
- Rust rustc 1.79.0

## 前提条件
- gitをインストール済み
- Rustをインストール済み
- javaをインストール済み(サーバーを起動するときに必要)

## 使用方法  
1. リポジトリをクローンします。  
    ```sh
    git clone https://github.com/mmiura-2351/mc-server-setup-script.git
    cd minecraft-server-setup-script
    ```

2. 必要な依存関係をインストールします。  
    ```sh
    cargo build
    ```

3. サーバーをセットアップします。以下のどちらかのコマンドを実行して、サーバーのバージョン、サーバータイプ（vanillaまたはforge）、およびディレクトリ名を指定します。  
    ### [方法1]指示に従ってサーバーをセットアップ  
    ```sh
    ./target/debug/minecraft_server_setup_script
    ```

    ### [方法2]引数を指定してサーバーをセットアップ  
    ```sh
    ./target/debug/minecraft_server_setup_script --version <バージョン> --server-type <サーバータイプ> --dir-name <ディレクトリ名>
    # -v <バージョン> -s <サーバータイプ> -d <ディレクトリ名>でも可
    ```
    また、引数は任意のため、`--version`、`--server-type`、`--dir-name`のうち必要なもののみを指定しても構いません。  


5. サーバーを起動します。  
    ```sh
    cd <ディレクトリ名>
    ./run.sh
    ```

## プログラムの詳細  
1. 引数の処理  
受け取ったコマンドライン引数をパースし、必要な情報（サーバーバージョン、サーバータイプ、ディレクトリ名など）を取得します。足りない情報がある場合は、標準入力からユーザーに入力を促します。

2. サーバーのセットアップ  
ユーザーが指定した引数に基づいて、Minecraftサーバーのセットアップを行います。必要なファイルやディレクトリを作成します。

3. サーバーのダウンロード  
ユーザーが指定したバージョン、サーバータイプに基づいて、サーバーをダウンロードします。  
このとき、バージョンが1.17以降かつ、サーバータイプがForgeの場合は、特殊な処理を行います。

4. サーバーのスタートスクリプトの作成  
Minecraftのサーバーはバージョンにより必要なJavaのバージョンが変わるため、対応したJavaのバージョンを指定してスタートスクリプトを作成します。

5. eula.txtの作成  
サーバーのライセンスに同意するために、eula.txtを作成します。
