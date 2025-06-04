package main

import (
	"fmt"
	"log"
	"os"
	"path/filepath"

	"github.com/BurntSushi/toml"
	"github.com/spf13/cobra"
)

type ToolConfig struct {
	Name string `toml:"name"`
	Exec string `toml:"exec"`
}

type TomlConfig struct {
	Tool []ToolConfig `toml:"tool"`
}

func getConfigs() TomlConfig {
	var res TomlConfig

	// 環境変数から設定ディレクトリを取得
	configDir, ok := os.LookupEnv("XDG_CONFIG_HOME")
	if !ok {
		configDir = filepath.Join(os.Getenv("HOME"), ".config")
	}

	// 設定ファイルのパスを構築
	configFile := filepath.Join(configDir, "custom-completion-zsh", "config.toml")

	_, err := toml.DecodeFile(configFile, &res)
	if err != nil {
		log.Fatalf("failed to decode TOML: %v", err)
	}

	return res
}

var rootCmd = &cobra.Command{
	Use:   "example",
	Long:  "A tool for zsh that automatically sets completion commands set by the user \nLinux only",
	Short: "Show example",
	RunE: func(cmd *cobra.Command, args []string) error {
		if len(args) == 0 {
			cmd.Help()
		}
		return nil
	},
}

var getConfigCmd = &cobra.Command{
	Use:   "getconfig",
	Long:  "Get config",
	Short: "Get config",
	RunE: func(cmd *cobra.Command, args []string) error {
		config := getConfigs()

		for _, tool := range config.Tool {
			fmt.Printf("tool name: %s exec command: %s\n", tool.Name, tool.Exec)
		}

		return nil
	},
}

func main() {
	err := rootCmd.Execute()
	if err != nil {
		os.Exit(1)
	}
}

func init() {
	rootCmd.AddCommand(getConfigCmd)
}
