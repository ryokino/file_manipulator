# ファイル操作ツール

このプロジェクトは、ファイルに対して様々な操作を行うことができるコマンドラインツールです。

## 機能

以下のコマンドが利用可能です：

### reverse
ファイルの内容を逆順にして新しいファイルに出力します。

```bash
cargo run --bin file_manipulator reverse <入力ファイルパス> <出力ファイルパス>
```

### copy
ファイルの内容を別のファイルにコピーします。

```bash
cargo run --bin file_manipulator copy <入力ファイルパス> <出力ファイルパス>
```

### duplicate-contents
ファイルの内容を指定回数分複製します。

```bash
cargo run --bin file_manipulator duplicate-contents <入力ファイルパス> <複製回数>
```

### replace-string
ファイル内の特定の文字列を別の文字列に置換します。

```bash
cargo run --bin file_manipulator replace-string <入力ファイルパス> <置換前の文字列> <置換後の文字列>
```

## 注意事項

- 入力ファイルは存在する必要があります
- 出力ファイルは自動的に作成されます（既に存在する場合は上書きされます）
- エラーが発生した場合は、適切なエラーメッセージが表示されます
