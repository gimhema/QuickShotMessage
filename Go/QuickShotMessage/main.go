package main

import (
	quickshotmessage "QuickShotMessage/QuickShotPackage"
	"fmt"
	"testing"
)

func main() {
	// Testing deserialization
	{
		fmt.Println("Running TestDeserialize...")
		t := &testing.T{}
		quickshotmessage.TestDeserialize(t)
		if t.Failed() {
			fmt.Println("TestDeserialize failed.")
		} else {
			fmt.Println("TestDeserialize passed.")
		}
	}

	// Testing data extraction
	{
		fmt.Println("Running TestExtractData...")
		t := &testing.T{}
		quickshotmessage.TestExtractData(t)
		if t.Failed() {
			fmt.Println("TestExtractData failed.")
		} else {
			fmt.Println("TestExtractData passed.")
		}
	}

	// Testing serialization
	{
		fmt.Println("Running TestSerialize...")
		t := &testing.T{}
		quickshotmessage.TestSerialize(t)
		if t.Failed() {
			fmt.Println("TestSerialize failed.")
		} else {
			fmt.Println("TestSerialize passed.")
		}
	}
}
