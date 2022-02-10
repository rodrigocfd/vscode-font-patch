package ini

type Section struct {
	Name    string
	Entries []Entry
}

func (me *Section) Value(name string) (*Entry, bool) {
	for i := range me.Entries {
		entry := &me.Entries[i]
		if entry.Key == name {
			return entry, true
		}
	}
	return nil, false
}
