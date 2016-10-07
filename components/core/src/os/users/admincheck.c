#include <windows.h>
#include <winnt.h>
#include <ShlObj.h>

#pragma comment (lib,"shell32.lib")

TOKEN_ELEVATION_TYPE s_elevationType = TokenElevationTypeDefault;
BOOL                 s_bIsAdmin = FALSE;

// returns 0 if an admin
// returns 1 if not
int GetElevationState()
{
    if (GetProcessElevation(&s_elevationType, &s_bIsAdmin)) {
        switch(s_elevationType) {
            // Default user or UAC is disabled
            case TokenElevationTypeDefault:  
                if (IsUserAnAdmin()) {
                    return 0;
                } else {
                    return 1;
                }
            break;
            // Process has been successfully elevated
            case TokenElevationTypeFull:
                if (IsUserAnAdmin()) {
                    return 0;
                } else {
                    return 1;
                }
            break; 
            // Process is running with limited privileges
            case TokenElevationTypeLimited:
                if (IsUserAnAdmin()) {
                    // Not sure what capabilities a filtered admin has
                    // So, for now, I'll treat that as non-admin
                    return 1;
                } else {
                    return 1;
                }
            break;
        }
        return 1;
    }
    return 1;
} 

BOOL GetProcessElevation(TOKEN_ELEVATION_TYPE* pElevationType, BOOL* pIsAdmin) {
   HANDLE hToken = NULL;
   DWORD dwSize;

   // Get current process token
   if (!OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &hToken))
      return(FALSE);

   BOOL bResult = FALSE;

   // Retrieve elevation type information
   if (GetTokenInformation(hToken, TokenElevationType,
      pElevationType, sizeof(TOKEN_ELEVATION_TYPE), &dwSize)) {
      // Create the SID corresponding to the Administrators group
      BYTE adminSID[SECURITY_MAX_SID_SIZE];
      dwSize = sizeof(adminSID);
      CreateWellKnownSid(WinBuiltinAdministratorsSid, NULL, &adminSID,
         &dwSize);

      if (*pElevationType == TokenElevationTypeLimited) {
         // Get handle to linked token (will have one if we are lua)
         HANDLE hUnfilteredToken = NULL;
         GetTokenInformation(hToken, TokenLinkedToken, (VOID*)
            &hUnfilteredToken, sizeof(HANDLE), &dwSize);

         // Check if this original token contains admin SID
         if (CheckTokenMembership(hUnfilteredToken, &adminSID, pIsAdmin)) {
            bResult = TRUE;
         }

         // Don't forget to close the unfiltered token
         CloseHandle(hUnfilteredToken);
      } else {
         *pIsAdmin = IsUserAnAdmin();
         bResult = TRUE;
      }
   }

   // Don't forget to close the process token
   CloseHandle(hToken);

   return(bResult);
}