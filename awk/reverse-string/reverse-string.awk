BEGIN {
    getline input_string
    n = split(input_string, arr,  "");

    result = "";
    for (i = n ; i > 0; i--) result = result arr[i];

    print result;
}

