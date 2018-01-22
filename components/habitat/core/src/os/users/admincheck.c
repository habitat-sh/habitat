#include <windows.h>
#include <winnt.h>
#include <ShlObj.h>

#pragma comment (lib,"shell32.lib")

TOKEN_ELEVATION_TYPE s_elevationType = TokenElevationTypeDefault;
BOOL                 s_bIsAdmin = FALSE;

// returns 0 if an admin and/or elevated
// returns 1 if a filtered user token (admin but UAC'd)
// returns 2 if a regular user
// returns 5 if none of the standard elevation types match
// returns 10 if GetProcessElevation failed
int GetUserTokenStatus()
{
    int user_token_status = 10;
    if (GetProcessElevation(&s_elevationType, &s_bIsAdmin)) {
        switch(s_elevationType) {
            // Default user or UAC is disabled
            case TokenElevationTypeDefault:  
                if (IsUserAnAdmin()) {
                    user_token_status = 0;
                } else {
                    user_token_status =  2;
                }
            break;
            // Process has been successfully elevated
            case TokenElevationTypeFull:
                if (IsUserAnAdmin()) {
                    user_token_status =  0;
                } else {
                    user_token_status =  0;
                }
            break; 
            // Process is running with limited privileges
            case TokenElevationTypeLimited:
                if (IsUserAnAdmin()) {
                    // Not sure what capabilities a filtered admin has
                    // So, for now, I'll treat that as non-admin
                    user_token_status =  3;
                } else {
                    user_token_status =  1;
                }
            break;
            default:
                user_token_status =  5;
        }
    }
    return user_token_status;
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