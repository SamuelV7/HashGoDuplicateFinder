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
	"os"
	"path/filepath"
	"strings"
)

type theMap struct {
	rootDir string
	hashMap map[hash.Hash][]string
}
type fileDetails struct {
	path     string
	fileInfo fs.FileInfo
}

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

func hashToHex(hash hash.Hash) string {
	return fmt.Sprintf("%x", hash.Sum(nil))
}

//traverse a directory, if it contains directory traverse them to
// add all these traversed paths into a list
//
func isFile(path string) bool {
	fileInfo, err := os.Stat(path)
	if err != nil {
		fmt.Println(err)
	}
	if !fileInfo.IsDir() {
		return true
	} else {
		return false
	}
}

func walkFileDirectory(thePaths string) []fileDetails {
	var listOfFiles []fileDetails
	_ = filepath.WalkDir(thePaths, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			return err
		}
		theInfo, _ := d.Info()
		fileDetailsInstance := fileDetails{
			path:     path,
			fileInfo: theInfo,
		}
		//if path is directory then don't add otherwise add
		if isFile(path) {
			listOfFiles = append(listOfFiles, fileDetailsInstance)
		}
		return nil
	})
	return listOfFiles
}

func hashMapFromListOfFiles(rootDir string, files []fileDetails) theMap {
	fileHashMap := make(map[hash.Hash][]string)
	for _, fileDetails := range files {
		fileHash := hashOfFile(fileDetails.path)
		if len(fileHashMap[fileHash]) == 0 {
			var tempSlice []string
			append(tempSlice, fileDetails.path)
		}
		fileHashMap[fileHash] = append(fileHashMap[fileHash], fileDetails.path)
	}
	return theMap{
		rootDir: rootDir,
		hashMap: fileHashMap,
	}
}

func formatNumbers(number int) string {
	p := message.NewPrinter(language.English)
	return p.Sprintf("%d\n", number)
}

func lengthOfListFormatted(files *[]fileDetails) string {
	return formatNumbers(len(*files))
}
func main() {
	testDir := "/Users/samuelvarghese/Documents"
	fileList := walkFileDirectory(testDir)
	for i, s := range fileList {
		fmt.Println(i, s.fileInfo.Size(), s.path)
	}
	fmt.Println("Scanned a total of " + lengthOfListFormatted(&fileList) + " files")
	fileMap := hashMapFromListOfFiles(testDir, fileList)
	for key, element := range fileMap.hashMap {
		key := hashToHex(key)
		duplicates := len(element)
		if duplicates > 1 {
			fmt.Println(key, " has duplicates ", duplicates)
		}
	}
}
