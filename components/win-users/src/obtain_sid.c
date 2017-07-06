// This function was copied from the snippet published on MSDH
// at https://msdn.microsoft.com/en-us/library/windows/desktop/aa379608(v=vs.85).aspx
#include <windows.h>

BOOL ObtainSid(HANDLE hToken, PSID *psid)

    {
    BOOL                    bSuccess = FALSE; // assume function will
                                                // fail
    DWORD                   dwIndex;
    DWORD                   dwLength = 0;
    TOKEN_INFORMATION_CLASS tic      = TokenGroups;
    PTOKEN_GROUPS           ptg      = NULL;

    __try
            {
            // 
            // determine the size of the buffer
    // 
            if (!GetTokenInformation(
            hToken,
            tic,
            (LPVOID)ptg,
            0,
            &dwLength
            ))
                {
                if (GetLastError() == ERROR_INSUFFICIENT_BUFFER)
                    {
                    ptg = (PTOKEN_GROUPS)HeapAlloc(
                        GetProcessHeap(),
                HEAP_ZERO_MEMORY,
                dwLength
                );
                    if (ptg == NULL)
                        __leave;
                    }
                else
                    __leave;
        }

            // 
            // obtain the groups the access token belongs to
            // 
            if (!GetTokenInformation(
                hToken,
            tic,
            (LPVOID)ptg,
            dwLength,
            &dwLength
            ))
                __leave;

            // 
            // determine which group is the logon sid
            // 
            for (dwIndex = 0; dwIndex < ptg->GroupCount; dwIndex++)
                {
            if ((ptg->Groups[dwIndex].Attributes & SE_GROUP_LOGON_ID)
                ==  SE_GROUP_LOGON_ID)
                    {
                    // 
                    // determine the length of the sid
                    // 
                    dwLength = GetLengthSid(ptg->Groups[dwIndex].Sid);

                    // 
                    // allocate a buffer for the logon sid
                    // 
                    *psid = (PSID)HeapAlloc(
                        GetProcessHeap(),
                HEAP_ZERO_MEMORY,
                dwLength
                );
                if (*psid == NULL)
                    __leave;

                // 
                // obtain a copy of the logon sid
                // 
                if (!CopySid(dwLength, *psid, ptg->Groups[dwIndex].Sid))
                    __leave;

                // 
                // break out of the loop because the logon sid has been
                // found
                // 
                break;
                }
            }

            // 
            // indicate success
            // 
            bSuccess = TRUE;
            }
    __finally
            {
            // 
    // free the buffer for the token group
    // 
            if (ptg != NULL)
                HeapFree(GetProcessHeap(), 0, (LPVOID)ptg);
            }

    return bSuccess;

}
