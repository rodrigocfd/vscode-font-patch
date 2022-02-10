package ini

import (
	"bufio"
	"os"
	"strings"
)

type File struct {
	Sections []Section
}

func Load(filePath string) (*File, error) {
	lines, err := loadLines(filePath)
	if err != nil {
		return nil, err
	}

	me := &File{}
	me.Sections = make([]Section, 0, 4) // arbitrary
	var curSection Section

	for _, line := range lines {
		if len(line) == 0 {
			continue // skip blank lines
		}

		if line[0] == '[' && line[len(line)-1] == ']' { // [section] ?
			if curSection.Name != "" {
				me.Sections = append(me.Sections, curSection)
			}
			curSection = Section{ // create a new section with the given name
				Name:    strings.TrimSpace(line[1 : len(line)-1]),
				Entries: make([]Entry, 0, 4), // arbitrary
			}

		} else if curSection.Name != "" {
			keyVal := strings.SplitN(line, "=", 2)
			curSection.Entries = append(curSection.Entries, Entry{
				Key: strings.TrimSpace(keyVal[0]),
				Val: strings.TrimSpace(keyVal[1]),
			})
		}
	}

	if curSection.Name != "" { // for the last section
		me.Sections = append(me.Sections, curSection)
	}

	return me, nil
}

func loadLines(filePath string) ([]string, error) {
	fin, err := os.Open(filePath)
	if err != nil {
		return nil, err
	}
	defer fin.Close()

	lines := make([]string, 10) // arbitrary

	scanner := bufio.NewScanner(fin)
	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}
	return lines, nil
}

func (me *File) Section(name string) (*Section, bool) {
	for i := range me.Sections {
		section := &me.Sections[i]
		if section.Name == name {
			return section, true
		}
	}
	return nil, false
}

func (me *File) Value(sectionName, valueName string) (*string, bool) {
	if section, ok := me.Section(sectionName); ok {
		if entry, ok := section.Value(valueName); ok {
			return &entry.Val, true
		}
	}
	return nil, false
}
