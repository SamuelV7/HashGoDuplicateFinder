package main

import (
	"crypto/sha256"
	"fmt"
	"golang.org/x/text/language"
	"golang.org/x/text/message"
	"hash"
	"io"
	"io/fs"
	"log"
	"path/filepath"
	"strings"
)

//calculate hash of large file by reading it into buffer
func hashOfFile(path string) hash.Hash {
	input := strings.NewReader(path)
	hash := sha256.New()
	if _, err := io.Copy(hash, input); err != nil {
		log.Fatal(err)
	}
	hash.Sum(nil)
	return hash
}

//traverse a directory, if it contains directory traverse them to
// add all these traversed paths into a list
//
func walkFileDirectory(thePaths string) []string {
	var listOfFiles []string
	_ = filepath.WalkDir(thePaths, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			return err
		}
		listOfFiles = append(listOfFiles, path)
		return nil
	})
	return listOfFiles
}
func formatNumbers(number int) string {
	p := message.NewPrinter(language.English)
	return p.Sprintf("%d\n", number)
}
func lengthOfListFormatted(files *[]string) string {
	return formatNumbers(len(*files))
}
func main() {
	testDir := "/Users/samuelvarghese/Downloads"
	fileList := walkFileDirectory(testDir)

	fmt.Println("Scanned a total of " + lengthOfListFormatted(&fileList) + " files")
	//for i, s := range fileList {
	//	fmt.Println(i, s)
	//}
}
