package main

import (
	"fmt"
	"log"
	"os"
	"os/exec"
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

func getConfigDir() string {

	// 環境変数から設定ディレクトリを取得
	configDir, ok := os.LookupEnv("XDG_CONFIG_HOME")
	if !ok {
		configDir = filepath.Join(os.Getenv("HOME"), ".config")
	}

	return configDir
}

func getConfigPath() string {
	configDir := getConfigDir()

	// 設定ファイルのパスを構築
	configFile := filepath.Join(configDir, "custom-completion-zsh", "config.toml")
	return configFile
}

func getConfigs() TomlConfig {
	var res TomlConfig

	// 設定ファイルのパスを構築
	// configFile := filepath.Join(configDir, "custom-completion-zsh", "config.toml")
	configFile := getConfigPath()

	_, err := toml.DecodeFile(configFile, &res)
	if err != nil {
		log.Fatalf("failed to decode TOML: %v", err)
	}

	return res
}

func getCompfilePath() string {

	// 環境変数から設定ディレクトリを取得
	configDir, ok := os.LookupEnv("XDG_DATA_HOME")
	if !ok {
		configDir = filepath.Join(os.Getenv("HOME"), ".local/share")
	}

	configDir = filepath.Join(configDir, "zsh", "custom-completion-zsh")

	return configDir
}

var rootCmd = &cobra.Command{
	Use:   "custom-completion-zsh",
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
			fmt.Printf("tool name: %s exec command: %s\n", tool.Name, tool.Exec) // TODO:文字を強調させる
		}

		return nil
	},
}

func cleanCompfile() {
	compfilePath := getCompfilePath()
	// exec.Command("/bin/zsh", "-i", "-c", "rm", "-rf", compfilePath).Run()
	os.RemoveAll(compfilePath)
	exec.Command("mkdir", "-p", compfilePath).Run()
}

var buildCmd = &cobra.Command{
	Use:   "build",
	Long:  "Build",
	Short: "Build",
	RunE: func(cmd *cobra.Command, args []string) error {
		compfilePath := getCompfilePath()
		cleanCompfile()

		for _, tool := range getConfigs().Tool {
			output, _ := exec.Command("/bin/zsh", "-i", "-c", tool.Exec).Output()
			err := os.WriteFile(filepath.Join(compfilePath, "_"+tool.Name), output, 0644)

			if err != nil {
				fmt.Println(err.Error())
				cleanCompfile()
				return nil
			}
		}

		fmt.Printf("Add %s to your fpathpath\n", compfilePath)

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
	rootCmd.AddCommand(buildCmd)
}
