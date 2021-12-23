package main

import (
	"crypto/sha256"
	"fmt"
	"golang.org/x/text/language"
	"golang.org/x/text/message"
	"io"
	"io/fs"
	"log"
	"os"
	"path/filepath"
)

type theMap struct {
	rootDir string
	hashMap map[string][]fileDetails
}
type fileDetails struct {
	path     string
	fileInfo fs.FileInfo
}

//calculate hash of large file by reading it into buffer
func hashOfFile(path string) []byte {
	file, err := os.Open(path)
	if err != nil {
		fmt.Println(err)
	}
	hash := sha256.New()
	if _, err := io.Copy(hash, file); err != nil {
		log.Fatal(err)
	}
	hash.Sum(nil)
	return hash.Sum(nil)
}

func hashToHex(hash []byte) string {
	return fmt.Sprintf("%x", hash)
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
	fileHashMap := make(map[string][]fileDetails)
	for _, fileDetails := range files {
		fileHash := hashOfFile(fileDetails.path)
		fileHashHex := hashToHex(fileHash)
		fileHashMap[fileHashHex] = append(fileHashMap[fileHashHex], fileDetails)
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
	//testDir := "/Users/samuelvarghese/Downloads"
	fileDir := os.Args[1]
	fileList := walkFileDirectory(fileDir)
	//for i, s := range fileList {
	//	fmt.Println(i, s.fileInfo.Size(), s.path)
	//}
	fmt.Println("Scanned a total of " + lengthOfListFormatted(&fileList) + " files")
	fileMap := hashMapFromListOfFiles(fileDir, fileList)
	for key, element := range fileMap.hashMap {
		duplicates := len(element)
		if duplicates > 1 {
			fmt.Println("duplicates ", duplicates, element[0].fileInfo.Name(), "     hash: ", key)
		}
	}
}
