## User

What approaches would be possible for storing the SSC preconfigurations as data like how [@spec.json](file:///workspaces/sed_manager/sed_spec/spec.json) stores a data base of known objects?

I see three use-cases for this at the moment:
- The virtual device's preconfiguration is currently defined in code in [@preconfig_admin.rs](file:///workspaces/sed_manager/sed_virtual_device/src/tper/opal_2/preconfig_admin.rs) and [@preconfig_locking.rs](file:///workspaces/sed_manager/sed_virtual_device/src/tper/opal_2/preconfig_locking.rs). This is verbose, and would get much worse if the remaining SSCs were added.
- Permission handling (future feature developed in the `backend-access-control` branch) needs to load the preconfiguration of the ACE table for authorities that can't read it or for ACEs that cannot be modified.
- Saving the virtual device's state between sessions, if the configuration is serializable as well, not just deserializable.

Ideally, the data should be human-readable. This basically means JSON with names as opposed to raw UIDs. Nevertheless, tokenization with the `Tokenize` and `Detokenize` traits is also interesting because it's very easy to add.
<context ref="file:///workspaces/sed_manager/sed_spec/spec.json">
# File outline for /workspaces/sed_manager/sed_spec/spec.json

Core [L2-274]
 Shared [L3-273]
  Authority [L4-22]
   Anybody [L5]
   Admins [L6]
   Makers [L7]
   MakerSymK [L8]
   MakerPuK [L9]
   SID [L10]
   TPerSign [L11]
   TPerExch [L12]
   AdminExch [L13]
   Issuers [L14]
   Editors [L15]
   Deleters [L16]
   Servers [L17]
   Reserve0 [L18]
   Reserve1 [L19]
   Reserve2 [L20]
   Reserve3 [L21]
  General [L23-26]
   Null [L24]
   CharSetUnrestricted [L25]
  InvokingID [L27-30]
   ThisSP [L28]
   SessionManager [L29]
  LockingInfo [L31-33]
   LockingInfo [L32]
  MBRControl [L34-36]
   MBRControl [L35]
  SMMethodID [L37-44]
   Properties [L38]
   StartSession [L39]
   SyncSession [L40]
   StartTrustedSession [L41]
   SyncTrustedSession [L42]
   CloseSession [L43]
  MethodID [L45-105]
   DeleteSP [L46]
   CreateTable [L47]
   Delete [L48]
   CreateRow [L49]
   DeleteRow [L50]
   OBSOLETE_0006 [L51]
   OBSOLETE_0007 [L52]
   Next [L53]
   GetFreeSpace [L54]
   GetFreeRows [L55]
   DeleteMethod [L56]
   OBSOLETE [L57]
   GetACL [L58]
   AddACE [L59]
   RemoveACE [L60]
   GenKey [L61]
   Reserved_0011 [L62]
   GetPackage [L63]
   SetPackage [L64]
   Get [L65]
   Set [L66]
   Authenticate [L67]
   IssueSP [L68]
   Reserved_0202 [L69]
   Reserved_0203 [L70]
   GetClock [L71]
   ResetClock [L72]
   SetClockHigh [L73]
   SetLagHigh [L74]
   SetClockLow [L75]
   SetLagLow [L76]
   IncrementCounter [L77]
   Random [L78]
   Salt [L79]
   DecryptInit [L80]
   Decrypt [L81]
   DecryptFinalize [L82]
   EncryptInit [L83]
   Encrypt [L84]
   EncryptFinalize [L85]
   HMACInit [L86]
   HMAC [L87]
   HMACFinalize [L88]
   HashInit [L89]
   Hash [L90]
   HashFinalize [L91]
   Sign [L92]
   Verify [L93]
   XOR [L94]
   AddLog [L95]
   CreateLog [L96]
   ClearLog [L97]
   FlushLog [L98]
   Reserved_0803 [L99]
   Revert [L100]
   RevertSP [L101]
   Activate [L102]
   Reactivate [L103]
   Erase [L104]
  SecretProtect [L106-137]
   C_PIN [L107]
   C_RSA_1024_6 [L108]
   C_RSA_1024_7 [L109]
   C_RSA_1024_8 [L110]
   C_RSA_1024_9 [L111]
   C_RSA_1024_A [L112]
   C_RSA_1024_B [L113]
   C_RSA_2048_6 [L114]
   C_RSA_2048_7 [L115]
   C_RSA_2048_8 [L116]
   C_RSA_2048_9 [L117]
   C_RSA_2048_A [L118]
   C_RSA_2048_B [L119]
   C_AES_128 [L120]
   C_AES_256 [L121]
   C_EC_160 [L122]
   C_EC_192 [L123]
   C_EC_224 [L124]
   C_EC_256 [L125]
   C_EC_384 [L126]
   C_EC_521 [L127]
   C_EC_163 [L128]
   C_EC_233 [L129]
   C_EC_283 [L130]
   C_HMAC_160 [L131]
   C_HMAC_256 [L132]
   C_HMAC_384 [L133]
   C_HMAC_512 [L134]
   K_AES_128 [L135]
   K_AES_256 [L136]
  SPInfo [L138-140]
   SPInfo [L139]
  SPTemplates [L141-149]
   Base [L142]
   Admin [L143]
   Clock [L144]
   Crypto [L145]
   Log [L146]
   Locking [L147]
   Reserved [L148]
  Table [L150-203]
   Table [L151]
   SPInfo [L152]
   SPTemplates [L153]
   Column [L154]
   Type [L155]
   MethodID [L156]
   AccessControl [L157]
   ACE [L158]
   Authority [L159]
   Certificates [L160]
   C_PIN [L161]
   C_RSA_1024 [L162]
   C_RSA_2048 [L163]
   C_AES_128 [L164]
   C_AES_256 [L165]
   C_EC_160 [L166]
   C_EC_192 [L167]
   C_EC_224 [L168]
   C_EC_256 [L169]
   C_EC_384 [L170]
   C_EC_521 [L171]
   C_EC_163 [L172]
   C_EC_233 [L173]
   C_EC_283 [L174]
   C_HMAC_160 [L175]
   C_HMAC_256 [L176]
   C_HMAC_384 [L177]
   C_HMAC_512 [L178]
   SecretProtect [L179]
   TPerInfo [L180]
   CryptoSuite [L181]
   Template [L182]
   SP [L183]
   ClockTime [L184]
   H_SHA_1 [L185]
   H_SHA_256 [L186]
   H_SHA_384 [L187]
   H_SHA_512 [L188]
   Log [L189]
   LogList [L190]
   LockingInfo [L191]
   Locking [L192]
   MBRControl [L193]
   MBR [L194]
   K_AES_128 [L195]
   K_AES_256 [L196]
   DataStore{n} [L197-202]
    base [L198]
    count [L199]
    step [L200]
    display_base [L201]
  TableID [L204-260]
   General [L205]
   InvokingID [L206]
   SMMethodID [L207]
   Table [L208]
   SPInfo [L209]
   SPTemplates [L210]
   Column [L211]
   Type [L212]
   MethodID [L213]
   AccessControl [L214]
   ACE [L215]
   Authority [L216]
   Certificates [L217]
   C_PIN [L218]
   C_RSA_1024 [L219]
   C_RSA_2048 [L220]
   C_AES_128 [L221]
   C_AES_256 [L222]
   C_EC_160 [L223]
   C_EC_192 [L224]
   C_EC_224 [L225]
   C_EC_256 [L226]
   C_EC_384 [L227]
   C_EC_521 [L228]
   C_EC_163 [L229]
   C_EC_233 [L230]
   C_EC_283 [L231]
   C_HMAC_160 [L232]
   C_HMAC_256 [L233]
   C_HMAC_384 [L234]
   C_HMAC_512 [L235]
   SecretProtect [L236]
   TPerInfo [L237]
   CryptoSuite [L238]
   Template [L239]
   SP [L240]
   ClockTime [L241]
   H_SHA_1 [L242]
   H_SHA_256 [L243]
   H_SHA_384 [L244]
   H_SHA_512 [L245]
   Log [L246]
   LogList [L247]
   LockingInfo [L248]
   Locking [L249]
   MBRControl [L250]
   MBR [L251]
   K_AES_128 [L252]
   K_AES_256 [L253]
   DataStore{n} [L254-259]
    base [L255]
    count [L256]
    step [L257]
    display_base [L258]
  Template [L261-269]
   Base [L262]
   Admin [L263]
   Clock [L264]
   Crypto [L265]
   Log [L266]
   Locking [L267]
   Reserved [L268]
  TPerInfo [L270-272]
   TPerInfo [L271]
DataStore [L275-298]
 Admin [L276-281]
  SP [L277-280]
   Admin [L278]
   Locking [L279]
 Locking [L282-297]
  ACE [L283-296]
   DataStore{n}_Get_All [L284-289]
    base [L285]
    count [L286]
    step [L287]
    display_base [L288]
   DataStore{n}_Set_All [L290-295]
    base [L291]
    count [L292]
    step [L293]
    display_base [L294]
Enterprise [L299-392]
 Admin [L300-323]
  ACE [L301-308]
   Anybody [L302]
   Makers [L303]
   SID [L304]
   SID_SetSelf [L305]
   MSID_Get [L306]
   SID_SetMakers [L307]
  Authority [L309-314]
   Anybody [L310]
   Makers [L311]
   MakerSymK [L312]
   SID [L313]
  C_PIN [L315-318]
   SID [L316]
   MSID [L317]
  SP [L319-322]
   Admin [L320]
   Locking [L321]
 Locking [L324-391]
  ACE [L325-348]
   Anybody [L326]
   BandMaster{n} [L327-331]
    base [L328]
    count [L329]
    display_base [L330]
   BandMaster{n}_SetSelf [L332-336]
    base [L333]
    count [L334]
    display_base [L335]
   BandMaster{n}_SetBand [L337-341]
    base [L338]
    count [L339]
    display_base [L340]
   EraseMaster [L342]
   EraseMaster_SetSelf [L343]
   AnyMaster [L344]
   BandMasters [L345]
   Anybody_GetBand [L346]
   Get_K_AES_Mode [L347]
  Authority [L349-358]
   Anybody [L350]
   BandMaster{n} [L351-355]
    base [L352]
    count [L353]
    display_base [L354]
   EraseMaster [L356]
   BandMasters [L357]
  C_PIN [L359-366]
   BandMaster{n} [L360-364]
    base [L361]
    count [L362]
    display_base [L363]
   EraseMaster [L365]
  Locking [L367-374]
   GlobalRange [L368]
   Band{n} [L369-373]
    base [L370]
    count [L371]
    display_base [L372]
  K_AES_128 [L375-382]
   GlobalRange_Key [L376]
   Band{n}_Key [L377-381]
    base [L378]
    count [L379]
    display_base [L380]
  K_AES_256 [L383-390]
   GlobalRange_Key [L384]
   Band{n}_Key [L385-389]
    base [L386]
    count [L387]
    display_base [L388]
KPIO [L393-470]
 Admin [L394-431]
  ACE [L395-406]
   Anybody [L396]
   Admin [L397]
   Set_Enabled [L398]
   C_PIN_SID_Get_NOPIN [L399]
   C_PIN_SID_Set_PIN [L400]
   C_PIN_MSID_Get_PIN [L401]
   C_PIN_Admins_Set_PIN [L402]
   TPerInfo_Set_ProgrammaticResetEnable [L403]
   SP_SID [L404]
   DataRemovalMechanism_Set_ActiveDataRemovalMechanism [L405]
  Authority [L407-417]
   Anybody [L408]
   Admins [L409]
   Makers [L410]
   SID [L411]
   Admin{n} [L412-416]
    base [L413]
    count [L414]
    display_base [L415]
  C_PIN [L418-426]
   SID [L419]
   MSID [L420]
   Admin{n} [L421-425]
    base [L422]
    count [L423]
    display_base [L424]
  SP [L427-430]
   Admin [L428]
   KeyPerIO [L429]
 KeyPerIO [L432-469]
  ACE [L433-452]
   Anybody [L434]
   Admin [L435]
   Anybody_Get_CommonName [L436]
   Admins_Set_CommonName [L437]
   ACE_Get_All [L438]
   ACE_Set_BooleanExpression [L439]
   Authority_Get_All [L440]
   Authority_Set_Enabled [L441]
   C_PIN_Admins_Get_All_NOPIN [L442]
   C_PIN_Admins_Set_PIN [L443]
   KPIOPolicies_Admin_Set [L444]
   KPIOPolicies_Admin_Get [L445]
   KeyTagAllocation_Admin_Set [L446]
   KeyTagAllocation_Admin_Get [L447]
   KeyEncryptionKey_Admin_Set [L448]
   KeyEncryptionKey_Admin_Get [L449]
   DataStore_Get_All [L450]
   DataStore_Set_All [L451]
  Authority [L453-461]
   Anybody [L454]
   Admins [L455]
   Admin{n} [L456-460]
    base [L457]
    count [L458]
    display_base [L459]
  C_PIN [L462-468]
   Admin{n} [L463-467]
    base [L464]
    count [L465]
    display_base [L466]
Opalite [L471-542]
 Admin [L472-494]
  ACE [L473-480]
   Anybody [L474]
   C_PIN_SID_Get_NOPIN [L475]
   C_PIN_SID_Set_PIN [L476]
   C_PIN_MSID_Get_PIN [L477]
   TPerInfo_Set_ProgrammaticResetEnable [L478]
   SP_SID [L479]
  Authority [L481-485]
   Anybody [L482]
   Admins [L483]
   SID [L484]
  C_PIN [L486-489]
   SID [L487]
   MSID [L488]
  SP [L490-493]
   Admin [L491]
   Locking [L492]
 Locking [L495-541]
  ACE [L496-518]
   Anybody [L497]
   Admin [L498]
   ACE_Get_All [L499]
   ACE_Set_BooleanExpression [L500]
   Authority_Get_All [L501]
   Authority_Set_Enabled [L502]
   C_PIN_Admins_Get_All_NOPIN [L503]
   C_PIN_Admins_Set_PIN [L504]
   C_PIN_User1_Set_PIN [L505]
   C_PIN_User2_Set_PIN [L506]
   K_AES_Mode [L507]
   K_AES_128_GlobalRange_GenKey [L508]
   K_AES_256_GlobalRange_GenKey [L509]
   Locking_GlobalRange_Get_RangeStartToActiveKey [L510]
   Locking_GlobalRange_Set_RdLocked [L511]
   Locking_GlobalRange_Set_WrLocked [L512]
   Locking_GlblRng_Admins_Set [L513]
   MBRControl_Admins_Set [L514]
   MBRControl_Set_DoneToDOR [L515]
   DataStore_Get_All [L516]
   DataStore_Set_All [L517]
  Authority [L519-526]
   Anybody [L520]
   Admins [L521]
   Admin1 [L522]
   Users [L523]
   User1 [L524]
   User2 [L525]
  C_PIN [L527-531]
   Admin1 [L528]
   User1 [L529]
   User2 [L530]
  Locking [L532-534]
   GlobalRange [L533]
  K_AES_128 [L535-537]
   GlobalRange_Key [L536]
  K_AES_256 [L538-540]
   GlobalRange_Key [L539]
Opal_2 [L543-694]
 Admin [L544-581]
  ACE [L545-556]
   Anybody [L546]
   Admin [L547]
   Set_Enabled [L548]
   C_PIN_SID_Get_NOPIN [L549]
   C_PIN_SID_Set_PIN [L550]
   C_PIN_MSID_Get_PIN [L551]
   C_PIN_Admins_Set_PIN [L552]
   TPerInfo_Set_ProgrammaticResetEnable [L553]
   SP_SID [L554]
   DataRemovalMechanism_Set_ActiveDataRemovalMechanism [L555]
  Authority [L557-567]
   Anybody [L558]
   Admins [L559]
   Makers [L560]
   SID [L561]
   Admin{n} [L562-566]
    base [L563]
    count [L564]
    display_base [L565]
  C_PIN [L568-576]
   SID [L569]
   MSID [L570]
   Admin{n} [L571-575]
    base [L572]
    count [L573]
    display_base [L574]
  SP [L577-580]
   Admin [L578]
   Locking [L579]
 Locking [L582-693]
  ACE [L583-641]
   Anybody [L584]
   Admin [L585]
   Anybody_Get_CommonName [L586]
   Admins_Set_CommonName [L587]
   ACE_Get_All [L588]
   ACE_Set_BooleanExpression [L589]
   Authority_Get_All [L590]
   Authority_Set_Enabled [L591]
   User{n}_Set_CommonName [L592-596]
    base [L593]
    count [L594]
    display_base [L595]
   C_PIN_Admins_Get_All_NOPIN [L597]
   C_PIN_Admins_Set_PIN [L598]
   C_PIN_User{n}_Set_PIN [L599-603]
    base [L600]
    count [L601]
    display_base [L602]
   K_AES_Mode [L604]
   K_AES_128_GlobalRange_GenKey [L605]
   K_AES_128_Range{n}_GenKey [L606-610]
    base [L607]
    count [L608]
    display_base [L609]
   K_AES_256_GlobalRange_GenKey [L611]
   K_AES_256_Range{n}_GenKey [L612-616]
    base [L613]
    count [L614]
    display_base [L615]
   Locking_GlobalRange_Get_RangeStartToActiveKey [L617]
   Locking_Range{n}_Get_RangeStartToActiveKey [L618-622]
    base [L619]
    count [L620]
    display_base [L621]
   Locking_GlobalRange_Set_RdLocked [L623]
   Locking_Range{n}_Set_RdLocked [L624-628]
    base [L625]
    count [L626]
    display_base [L627]
   Locking_GlobalRange_Set_WrLocked [L629]
   Locking_Range{n}_Set_WrLocked [L630-634]
    base [L631]
    count [L632]
    display_base [L633]
   Locking_GlblRng_Admins_Set [L635]
   Locking_Admins_RangeStartToLOR [L636]
   MBRControl_Admins_Set [L637]
   MBRControl_Set_DoneToDOR [L638]
   DataStore_Get_All [L639]
   DataStore_Set_All [L640]
  Authority [L642-656]
   Anybody [L643]
   Admins [L644]
   Admin{n} [L645-649]
    base [L646]
    count [L647]
    display_base [L648]
   Users [L650]
   User{n} [L651-655]
    base [L652]
    count [L653]
    display_base [L654]
  C_PIN [L657-668]
   Admin{n} [L658-662]
    base [L659]
    count [L660]
    display_base [L661]
   User{n} [L663-667]
    base [L664]
    count [L665]
    display_base [L666]
  Locking [L669-676]
   GlobalRange [L670]
   Range{n} [L671-675]
    base [L672]
    count [L673]
    display_base [L674]
  K_AES_128 [L677-684]
   GlobalRange_Key [L678]
   Range{n}_Key [L679-683]
    base [L680]
    count [L681]
    display_base [L682]
  K_AES_256 [L685-692]
   GlobalRange_Key [L686]
   Range{n}_Key [L687-691]
    base [L688]
    count [L689]
    display_base [L690]
PSID [L695-713]
 Admin [L696-711]
  ACE [L697-700]
   C_PIN_Get_PSID_NoPIN [L698]
   SP_PSID [L699]
  Authority [L701-703]
   PSID [L702]
  C_PIN [L704-706]
   PSID [L705]
  SP [L707-710]
   Admin [L708]
   Locking [L709]
 Locking [L712]
Pyrite_2 [L714-798]
 Admin [L715-751]
  ACE [L716-727]
   Anybody [L717]
   Admin [L718]
   Set_Enabled [L719]
   C_PIN_SID_Get_NOPIN [L720]
   C_PIN_SID_Set_PIN [L721]
   C_PIN_MSID_Get_PIN [L722]
   C_PIN_Admins_Set_PIN [L723]
   TPerInfo_Set_ProgrammaticResetEnable [L724]
   SP_SID [L725]
   DataRemovalMechanism_Set_ActiveDataRemovalMechanism [L726]
  Authority [L728-737]
   Anybody [L729]
   Admins [L730]
   SID [L731]
   Admin{n} [L732-736]
    base [L733]
    count [L734]
    display_base [L735]
  C_PIN [L738-746]
   SID [L739]
   MSID [L740]
   Admin{n} [L741-745]
    base [L742]
    count [L743]
    display_base [L744]
  SP [L747-750]
   Admin [L748]
   Locking [L749]
 Locking [L752-797]
  ACE [L753-772]
   Anybody [L754]
   Admin [L755]
   ACE_Get_All [L756]
   ACE_Set_BooleanExpression [L757]
   Authority_Get_All [L758]
   Authority_Set_Enabled [L759]
   C_PIN_Admins_Get_All_NOPIN [L760]
   C_PIN_Admins_Set_PIN [L761]
   C_PIN_User1_Set_PIN [L762]
   C_PIN_User2_Set_PIN [L763]
   Locking_GlobalRange_Get_RangeStartToActiveKey [L764]
   Locking_GlobalRange_Set_RdLocked [L765]
   Locking_GlobalRange_Set_WrLocked [L766]
   Locking_GlblRng_Admins_Set [L767]
   MBRControl_Admins_Set [L768]
   MBRControl_Set_DoneToDOR [L769]
   DataStore_Get_All [L770]
   DataStore_Set_All [L771]
  Authority [L773-784]
   Anybody [L774]
   Admins [L775]
   Admin{n} [L776-780]
    base [L777]
    count [L778]
    display_base [L779]
   Users [L781]
   User1 [L782]
   User2 [L783]
  C_PIN [L785-793]
   Admin{n} [L786-790]
    base [L787]
    count [L788]
    display_base [L789]
   User1 [L791]
   User2 [L792]
  Locking [L794-796]
   GlobalRange [L795]
Ruby [L799-949]
 Admin [L800-836]
  ACE [L801-811]
   Anybody [L802]
   Admin [L803]
   Set_Enabled [L804]
   C_PIN_SID_Get_NOPIN [L805]
   C_PIN_SID_Set_PIN [L806]
   C_PIN_MSID_Get_PIN [L807]
   C_PIN_Admins_Set_PIN [L808]
   TPerInfo_Set_ProgrammaticResetEnable [L809]
   SP_SID [L810]
  Authority [L812-822]
   Anybody [L813]
   Admins [L814]
   Makers [L815]
   SID [L816]
   Admin{n} [L817-821]
    base [L818]
    count [L819]
    display_base [L820]
  C_PIN [L823-831]
   SID [L824]
   MSID [L825]
   Admin{n} [L826-830]
    base [L827]
    count [L828]
    display_base [L829]
  SP [L832-835]
   Admin [L833]
   Locking [L834]
 Locking [L837-948]
  ACE [L838-896]
   Anybody [L839]
   Admin [L840]
   Anybody_Get_CommonName [L841]
   Admins_Set_CommonName [L842]
   ACE_Get_All [L843]
   ACE_Set_BooleanExpression [L844]
   Authority_Get_All [L845]
   Authority_Set_Enabled [L846]
   User{n}_Set_CommonName [L847-851]
    base [L848]
    count [L849]
    display_base [L850]
   C_PIN_Admins_Get_All_NOPIN [L852]
   C_PIN_Admins_Set_PIN [L853]
   C_PIN_User{n}_Set_PIN [L854-858]
    base [L855]
    count [L856]
    display_base [L857]
   K_AES_Mode [L859]
   K_AES_128_GlobalRange_GenKey [L860]
   K_AES_128_Range{n}_GenKey [L861-865]
    base [L862]
    count [L863]
    display_base [L864]
   K_AES_256_GlobalRange_GenKey [L866]
   K_AES_256_Range{n}_GenKey [L867-871]
    base [L868]
    count [L869]
    display_base [L870]
   Locking_GlobalRange_Get_RangeStartToActiveKey [L872]
   Locking_Range{n}_Get_RangeStartToActiveKey [L873-877]
    base [L874]
    count [L875]
    display_base [L876]
   Locking_GlobalRange_Set_RdLocked [L878]
   Locking_Range{n}_Set_RdLocked [L879-883]
    base [L880]
    count [L881]
    display_base [L882]
   Locking_GlobalRange_Set_WrLocked [L884]
   Locking_Range{n}_Set_WrLocked [L885-889]
    base [L886]
    count [L887]
    display_base [L888]
   Locking_GlblRng_Admins_Set [L890]
   Locking_Admins_RangeStartToLOR [L891]
   MBRControl_Admins_Set [L892]
   MBRControl_Set_DoneToDOR [L893]
   DataStore_Get_All [L894]
   DataStore_Set_All [L895]
  Authority [L897-911]
   Anybody [L898]
   Admins [L899]
   Admin{n} [L900-904]
    base [L901]
    count [L902]
    display_base [L903]
   Users [L905]
   User{n} [L906-910]
    base [L907]
    count [L908]
    display_base [L909]
  C_PIN [L912-923]
   Admin{n} [L913-917]
    base [L914]
    count [L915]
    display_base [L916]
   User{n} [L918-922]
    base [L919]
    count [L920]
    display_base [L921]
  Locking [L924-931]
   GlobalRange [L925]
   Range{n} [L926-930]
    base [L927]
    count [L928]
    display_base [L929]
  K_AES_128 [L932-939]
   GlobalRange_Key [L933]
   Range{n}_Key [L934-938]
    base [L935]
    count [L936]
    display_base [L937]
  K_AES_256 [L940-947]
   GlobalRange_Key [L941]
   Range{n}_Key [L942-946]
    base [L943]
    count [L944]
    display_base [L945]

Showing symbols 1-796 (total symbols: 796)

</context>
<context ref="file:///workspaces/sed_manager/sed_virtual_device/src/tper/opal_2/preconfig_admin.rs">
//L-----------------------------------------------------------------------------
//L Copyright (C) Péter Kardos
//L Please refer to the full license distributed with this software.
//L-----------------------------------------------------------------------------

use std::ops::Range;

use sed_spec::ace_expr;
use sed_spec::objects::{
    AccessControl, AccessControlRef, Ace, Authority, AuthorityRef, CPin, SecurityProvider, TableDesc,
};
use sed_spec::preconfig::core::shared::invoking_id::THIS_SP;
use sed_spec::preconfig::opal_2::admin::{ace, authority, c_pin, sp};
use sed_spec::preconfig::psid;
use sed_spec::types::{AuthMethod, LifeCycleState};

use crate::tper::Admin;
use crate::tper::preconfig_shared::{AllColumns, INITIAL_SID_PASSWORD, IntoTable, PSID_PASSWORD};
use crate::tper::security_provider::Table;

const ADMINS: Range<usize> = 0..4;

pub fn preconfig() -> Admin {
    Admin {
        uid: sp::ADMIN,
        access_control: access_control(),
        ace: ace(),
        authority: authority(),
        c_pin: c_pin(),
        sp: sp(),
        table: table(),
    }
}

pub fn access_control() -> Table<AccessControl> {
    use sed_spec::preconfig::core::shared::method_id::*;
    use sed_spec::preconfig::core::shared::table_id::*;

    let fixed = [
        // Table
        (
            AccessControlRef { invoking_id: TABLE.into(), method_id: NEXT },
            AccessControl { acl: vec![ace::ANYBODY], ..Default::default() },
        ),
        (
            AccessControlRef { invoking_id: TABLE.into(), method_id: GET },
            AccessControl { acl: vec![ace::ANYBODY], ..Default::default() },
        ),
        // AccessControl
        (
            AccessControlRef { invoking_id: ACCESS_CONTROL.into(), method_id: GET_ACL },
            AccessControl { acl: vec![ace::ANYBODY], ..Default::default() },
        ),
        // ACE
        (
            AccessControlRef { invoking_id: ACE.into(), method_id: NEXT },
            AccessControl { acl: vec![ace::ANYBODY], ..Default::default() },
        ),
        (
            AccessControlRef { invoking_id: ACE.into(), method_id: GET },
            AccessControl { acl: vec![ace::ANYBODY], ..Default::default() },
        ),
        // Authority
        (
            AccessControlRef { invoking_id: AUTHORITY.into(), method_id: NEXT },
            AccessControl { acl: vec![ace::ANYBODY], ..Default::default() },
        ),
        (
            AccessControlRef { invoking_id: AUTHORITY.into(), method_id: GET },
            AccessControl { acl: vec![ace::ANYBODY], ..Default::default() },
        ),
        // C_PIN
        (
            AccessControlRef { invoking_id: C_PIN.into(), method_id: NEXT },
            AccessControl { acl: vec![ace::ANYBODY], ..Default::default() },
        ),
        (
            AccessControlRef { invoking_id: c_pin::SID.into(), method_id: GET },
            AccessControl { acl: vec![ace::C_PIN_SID_GET_NOPIN], ..Default::default() },
        ),
        (
            AccessControlRef { invoking_id: c_pin::SID.into(), method_id: SET },
            AccessControl { acl: vec![ace::C_PIN_SID_SET_PIN], ..Default::default() },
        ),
        (
            AccessControlRef { invoking_id: c_pin::MSID.into(), method_id: GET },
            AccessControl { acl: vec![ace::C_PIN_MSID_GET_PIN], ..Default::default() },
        ),
        // SP
        (
            AccessControlRef { invoking_id: THIS_SP, method_id: AUTHENTICATE },
            AccessControl { acl: vec![ace::ANYBODY], ..Default::default() },
        ),
        (
            AccessControlRef { invoking_id: THIS_SP, method_id: RANDOM },
            AccessControl { acl: vec![ace::ANYBODY], ..Default::default() },
        ),
        (
            AccessControlRef { invoking_id: SP.into(), method_id: NEXT },
            AccessControl { acl: vec![ace::ANYBODY], ..Default::default() },
        ),
        (
            AccessControlRef { invoking_id: SP.into(), method_id: GET },
            AccessControl { acl: vec![ace::ANYBODY], ..Default::default() },
        ),
        (
            AccessControlRef { invoking_id: SP.into(), method_id: REVERT },
            AccessControl { acl: vec![ace::SP_SID, ace::ADMIN, psid::admin::ace::SP_PSID], ..Default::default() },
        ),
        (
            AccessControlRef { invoking_id: SP.into(), method_id: ACTIVATE },
            AccessControl { acl: vec![ace::SP_SID], ..Default::default() },
        ),
    ];

    let admins = ADMINS.map(|admin_idx| {
        // Authority
        [
            (
                AccessControlRef { invoking_id: authority::ADMIN.get(admin_idx).unwrap().into(), method_id: SET },
                AccessControl { acl: vec![ace::SET_ENABLED], ..Default::default() },
            ),
            (
                AccessControlRef { invoking_id: c_pin::ADMIN.get(admin_idx).unwrap().into(), method_id: GET },
                AccessControl { acl: vec![ace::C_PIN_SID_GET_NOPIN], ..Default::default() },
            ),
            (
                AccessControlRef { invoking_id: c_pin::ADMIN.get(admin_idx).unwrap().into(), method_id: SET },
                AccessControl { acl: vec![ace::C_PIN_ADMINS_SET_PIN], ..Default::default() },
            ),
        ]
    });

    fixed.into_iter().chain(admins.into_iter().flatten()).collect()
}

pub fn ace() -> Table<Ace> {
    [
        // Base ACEs
        Ace {
            uid: Some(ace::ANYBODY),
            boolean_expr: Some(ace_expr!((authority::ANYBODY))),
            columns: Some((0..32).collect()),
            ..Default::default()
        },
        Ace {
            uid: Some(ace::ADMIN),
            boolean_expr: Some(ace_expr!((authority::ADMINS))),
            columns: Some((0..32).collect()),
            ..Default::default()
        },
        // Authority table
        Ace {
            uid: Some(ace::SET_ENABLED),
            boolean_expr: Some(ace_expr!((authority::SID))),
            columns: Some([Authority::ENABLED].into()),
            ..Default::default()
        },
        // C_PIN table
        Ace {
            uid: Some(ace::C_PIN_SID_GET_NOPIN),
            boolean_expr: Some(ace_expr!((authority::ADMINS) (authority::SID) ||)),
            columns: Some(
                [
                    CPin::UID,
                    CPin::CHAR_SET,
                    CPin::TRY_LIMIT,
                    CPin::TRIES,
                    CPin::PERSISTENCE,
                ]
                .into(),
            ),
            ..Default::default()
        },
        Ace {
            uid: Some(ace::C_PIN_SID_SET_PIN),
            boolean_expr: Some(ace_expr!((authority::SID))),
            columns: Some([CPin::PIN].into()),
            ..Default::default()
        },
        Ace {
            uid: Some(ace::C_PIN_MSID_GET_PIN),
            boolean_expr: Some(ace_expr!((authority::ANYBODY))),
            columns: Some([CPin::UID, CPin::PIN].into()),
            ..Default::default()
        },
        Ace {
            uid: Some(ace::C_PIN_ADMINS_SET_PIN),
            boolean_expr: Some(ace_expr!((authority::ADMINS) (authority::SID) ||)),
            columns: Some([CPin::PIN].into()),
            ..Default::default()
        },
        // SP
        Ace {
            uid: Some(ace::SP_SID),
            boolean_expr: Some(ace_expr!((authority::SID))),
            columns: Some(SecurityProvider::all_columns().collect()),
            ..Default::default()
        },
        Ace {
            uid: Some(psid::admin::ace::SP_PSID),
            boolean_expr: Some(ace_expr!((psid::admin::authority::PSID))),
            columns: Some(SecurityProvider::all_columns().collect()),
            ..Default::default()
        },
    ]
    .into_table()
    .expect("object missing an UID")
}

pub fn authority() -> Table<Authority> {
    let fixed = [
        Authority {
            uid: Some(authority::ANYBODY),
            name: Some("Anybody".into()),
            is_class: Some(false),
            class: Some(AuthorityRef::null()),
            ..Default::default()
        },
        Authority {
            uid: Some(authority::ADMINS),
            name: Some("Admins".into()),
            is_class: Some(true),
            class: Some(AuthorityRef::null()),
            ..Default::default()
        },
        Authority {
            uid: Some(authority::MAKERS),
            name: Some("Makers".into()),
            is_class: Some(true),
            class: Some(AuthorityRef::null()),
            ..Default::default()
        },
        Authority {
            uid: Some(authority::SID),
            name: Some("SID".into()),
            is_class: Some(false),
            class: Some(AuthorityRef::null()),
            operation: Some(AuthMethod::Password),
            credential: Some(c_pin::SID.into()),
            ..Default::default()
        },
        Authority {
            uid: Some(psid::admin::authority::PSID),
            name: Some("PSID".into()),
            is_class: Some(false),
            class: Some(AuthorityRef::null()),
            operation: Some(AuthMethod::Password),
            credential: Some(psid::admin::c_pin::PSID.into()),
            ..Default::default()
        },
    ];

    let admins = ADMINS.map(|admin_idx| Authority {
        uid: Some(authority::ADMIN.get(admin_idx).unwrap()),
        name: Some(format!("Admin{}", admin_idx)),
        enabled: Some(false),
        is_class: Some(false),
        class: Some(authority::ADMINS),
        operation: Some(AuthMethod::Password),
        credential: Some(c_pin::ADMIN.get(admin_idx).unwrap().into()),
        ..Default::default()
    });

    fixed.into_iter().chain(admins).into_table().expect("object missing an UID")
}

pub fn c_pin() -> Table<CPin> {
    let fixed = [
        CPin { uid: Some(c_pin::SID), pin: Some(INITIAL_SID_PASSWORD), ..Default::default() },
        CPin { uid: Some(c_pin::MSID), pin: Some(INITIAL_SID_PASSWORD), ..Default::default() },
        CPin { uid: Some(psid::admin::c_pin::PSID), pin: Some(PSID_PASSWORD), ..Default::default() },
    ];

    let admins = ADMINS.map(|admin_idx| CPin {
        uid: Some(c_pin::ADMIN.get(admin_idx).unwrap()),
        pin: Some(b"random_password".as_slice().into()),
        ..Default::default()
    });

    fixed.into_iter().chain(admins).into_table().expect("object missing an UID")
}

pub fn sp() -> Table<SecurityProvider> {
    let fixed = [
        SecurityProvider {
            uid: Some(sp::ADMIN),
            name: Some("Admin".into()),
            life_cycle_state: Some(LifeCycleState::Manufactured),
            ..Default::default()
        },
        SecurityProvider {
            uid: Some(sp::LOCKING),
            name: Some("Locking".into()),
            life_cycle_state: Some(LifeCycleState::ManufacturedInactive),
            ..Default::default()
        },
    ];

    fixed.into_table().expect("object missing an UID")
}

pub fn table() -> Table<TableDesc> {
    [].into_table().expect("object missing an UID")
}

</context>
<context ref="file:///workspaces/sed_manager/sed_virtual_device/src/tper/opal_2/preconfig_locking.rs">
# File outline for /workspaces/sed_manager/sed_virtual_device/src/tper/opal_2/preconfig_locking.rs

const ADMINS [L23]
const USERS [L24]
const RANGES [L25]
const MBR_SIZE [L26]
const DATA_STORE_SIZE [L27]
pub fn preconfig [L29-44]
pub fn access_control [L46-332]
pub fn ace [L334-561]
pub fn authority [L563-619]
pub fn c_pin [L621-635]
fn random_key [L637-641]
pub fn k_aes_256 [L643-653]
pub fn locking [L655-684]
fn locking_info_ext [L686-701]
pub fn mbr_control [L703-712]
pub fn table [L714-732]

Showing symbols 1-16 (total symbols: 16)

</context>

**Tool Call: cd /workspaces/sed_manager; ls sed_spec/src sed_spec/src/* sed_spec_codegen/src sed_virtual_device/src/tper; cat sed_spec/build.rs; sed -n 1,80p sed_virtual_device/src/tper/preconfig_shared.rs; git branch -a; grep -rn "serde" --include=Cargo.toml . | head -20**
Status: Completed

Terminal:
```
sed_spec/src/lib.rs
sed_spec/src/lookup.rs
sed_spec/src/path.rs
sed_spec/src/preconfig.rs

sed_spec/src:
lib.rs
lookup.rs
methods
objects
path.rs
preconfig.rs
types

sed_spec/src/methods:
call.rs
cell_block.rs
method_param.rs
mod.rs
properties.rs
result.rs
session.rs
session_manager.rs
status.rs
token_stream.rs

sed_spec/src/objects:
access_control.rs
ace.rs
authority.rs
c_pin.rs
k_aes_256.rs
locking_info.rs
locking_range.rs
mbr_control.rs
mod.rs
reference.rs
security_provider.rs
table_desc.rs

sed_spec/src/types:
ace_operand.rs
adv_key_mode.rs
auth_method.rs
boolean_op.rs
date.rs
encryption_support.rs
general_status.rs
hash_protocol.rs
keys_available_condition.rs
last_reenc_status.rs
life_cycle_state.rs
log_select.rs
messaging_type.rs
mod.rs
reencrypt_request.rs
reencrypt_state.rs
reset_type.rs
symmetric_mode_media.rs
table_kind.rs
type.rs
verify_mode.rs

sed_spec_codegen/src:
lib.rs

sed_virtual_device/src/tper:
admin.rs
locking.rs
mod.rs
opal_2
preconfig_shared.rs
security_provider.rs
//L-----------------------------------------------------------------------------
//L Copyright (C) Péter Kardos
//L Please refer to the full license distributed with this software.
//L-----------------------------------------------------------------------------

use std::fs::File;
use std::io::{Read as _, Write as _};
use std::path::PathBuf;

fn main() -> Result<(), ()> {
    let spec_path = "spec.json";
    let mut spec_file = File::open(spec_path).unwrap();
    let mut spec_content = String::new();
    spec_file.read_to_string(&mut spec_content).unwrap();

    let generated_spec = sed_spec_codegen::generate_spec(&spec_content).unwrap();

    let mut out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    out_path.push("spec.rs");
    let mut out_file = File::create(out_path).unwrap();
    out_file.write_all(generated_spec.as_bytes()).unwrap();
    println!("cargo::rerun-if-changed={spec_path}");
    Ok(())
}
//L-----------------------------------------------------------------------------
//L Copyright (C) Péter Kardos
//L Please refer to the full license distributed with this software.
//L-----------------------------------------------------------------------------

use sed_packet::{MaxBytes, Object, ObjectUid};

use crate::tper::security_provider::Table;

// This is very annoying, unsafe should be absolutely unnecessary here, and
// this should be rewritten if proper const programming are available in Rust.
pub const INITIAL_SID_PASSWORD: MaxBytes<32> =
    unsafe { MaxBytes::from_const_with_len_unchecked(*b"password                        ", 8) };
pub const PSID_PASSWORD: MaxBytes<32> =
    unsafe { MaxBytes::from_const_with_len_unchecked(*b"recovery                        ", 8) };

pub trait AllColumns {
    fn all_columns() -> impl Iterator<Item = u16>;
}

impl<O: Object> AllColumns for O {
    fn all_columns() -> impl Iterator<Item = u16> {
        0..O::FIELD_COUNT
    }
}

pub trait IntoTable<O: Object> {
    fn into_table(self) -> Option<Table<O>>;
}

impl<C> IntoTable<<C as IntoIterator>::Item> for C
where
    C: IntoIterator,
    <C as IntoIterator>::Item: Object + ObjectUid,
    <<C as IntoIterator>::Item as Object>::Ref: Ord,
{
    fn into_table(self) -> Option<Table<<C as IntoIterator>::Item>> {
        self.into_iter().map(|object| object.uid().map(|uid| (uid, object))).collect()
    }
}
warning: ignoring broken ref refs/remotes/origin/HEAD
  backend-access-control
  enterprise-virtual-device
  master
  virtual-device-and-spec-improvements
* virtual-device-ssc-features
  remotes/origin/enterprise-virtual-device
  remotes/origin/master
  remotes/origin/virtual-device-and-spec-improvements
Cargo.toml:56:serde = { version = "1.0.228", features = ["derive"] }
Cargo.toml:57:serde_json = { version = "1.0.149" }
sed_spec_codegen/Cargo.toml:11:serde_json.workspace = true
sed_spec_codegen/Cargo.toml:12:serde.workspace = true
sed_telemetry/Cargo.toml:22:serde_json.workspace = true
```


**Tool Call: cd /workspaces/sed_manager; cat sed_spec/src/objects/ace.rs | sed -n 1,80p; cat sed_spec/src/lookup.rs | head -60; cat sed_spec/src/path.rs | head -60; sed -n 1,80p sed_virtual_device/src/tper/security_provider.rs; git log --oneline master..backend-access-control | head; git diff --stat master...backend-access-control | tail -30**
Status: Completed

Terminal:
```
//L-----------------------------------------------------------------------------
//L Copyright (C) Péter Kardos
//L Please refer to the full license distributed with this software.
//L-----------------------------------------------------------------------------

use std::collections::HashSet;

use sed_packet::ObjectUid;
use sed_spec_macros::{DetokenizeStruct, FieldList, Object, TokenizeField, TokenizeStruct};

use crate::objects::{AceRef, AuthorityRef};
use crate::preconfig::core::shared::table_id;
use crate::types::{AceOperand, BooleanOp};

#[macro_export]
macro_rules! ace_operand {
    (||) => {
        ::sed_spec::types::AceOperand::BooleanOp(::sed_spec::types::BooleanOp::Or)
    };
    (&&) => {
        ::sed_spec::types::AceOperand::BooleanOp(::sed_spec::types::BooleanOp::And)
    };
    (!) => {
        ::sed_spec::types::AceOperand::BooleanOp(::sed_spec::types::BooleanOp::Not)
    };
    ($authority:expr) => {
        ::sed_spec::types::AceOperand::Authority($authority)
    };
}

#[macro_export]
macro_rules! ace_expr {
    ($($operand:tt)*) => {
        Vec::<$crate::types::AceOperand>::from(vec![$($crate::objects::ace_operand!($operand)),*])
    };
}

pub use ace_expr;
pub use ace_operand;

#[derive(Debug, Clone, Default, PartialEq, Eq, Object, TokenizeStruct, DetokenizeStruct, FieldList, TokenizeField)]
#[object(table = table_id::ACE)]
pub struct Ace {
    pub uid: Option<AceRef>,
    pub name: Option<String>,
    pub common_name: Option<String>,
    pub boolean_expr: Option<Vec<AceOperand>>,
    pub columns: Option<HashSet<u16>>,
}

impl ObjectUid for Ace {
    fn uid(&self) -> Option<Self::Ref> {
        self.uid
    }

    fn set_uid(&mut self, uid: Option<Self::Ref>) {
        self.uid = uid;
    }
}

pub trait AceExpr {
    fn eval(&self, authenticated: impl IntoIterator<Item = AuthorityRef>) -> Option<bool>;
    fn allow_authority(&self, authority: AuthorityRef) -> Option<Vec<AceOperand>>;
    fn deny_authority(&self, authority: AuthorityRef) -> Option<Vec<AceOperand>>;
    fn normalize(&self) -> Option<Vec<AceOperand>>;
}

impl<Sequence> AceExpr for Sequence
where
    for<'seq> &'seq Sequence: IntoIterator<Item = &'seq AceOperand>,
{
    fn eval(&self, authenticated: impl IntoIterator<Item = AuthorityRef>) -> Option<bool> {
        let mut stack = Vec::<bool>::new();
        let authenticated: HashSet<_> = authenticated.into_iter().collect();
        for item in self.into_iter() {
            match item {
                AceOperand::Authority(authority) => {
                    stack.push(authenticated.contains(authority));
                }
                AceOperand::BooleanOp(BooleanOp::And) => {
//L-----------------------------------------------------------------------------
//L Copyright (C) Péter Kardos
//L Please refer to the full license distributed with this software.
//L-----------------------------------------------------------------------------

use std::ops::{Add, Sub};

use sed_packet::discovery::{FeatureDescriptor, SscFeatureDescriptor};
use sed_packet::{ObjectRef, TableRef, Uid, UidRange};

use crate::path::Path;

// These tables and types are hardcoded here because the generated specification
// is not available here.
pub const SP_TABLE: TableRef = TableRef::new(0x00000205_00000000);
pub const META_TABLE: TableRef = TableRef::new(0xFFFFFFFF_00000000);
pub(crate) type SpRef = ObjectRef<{ SP_TABLE.to_u64() }>;

/// A contiguous range of UIDs whose names follow the pattern `{prefix}{n}{suffix}`,
/// where `n` is a display index starting at `display_base`.
pub struct UidRangeEntry<U> {
    pub range: UidRange<U>,
    pub prefix: &'static str,
    pub suffix: &'static str,
    pub display_base: u64,
}

impl<U> UidRangeEntry<U>
where
    U: Copy + PartialOrd + Add<u32, Output = U> + Sub<U, Output = i64>,
{
    pub fn by_name(&self, name: &str) -> Option<U> {
        let digits = name.strip_prefix(self.prefix)?.strip_suffix(self.suffix)?;
        let display_index: u64 = digits.parse().ok()?;
        let index = display_index.checked_sub(self.display_base)?;
        self.range.get(usize::try_from(index).ok()?)
    }

    pub fn by_uid(&self, uid: U) -> Option<String> {
        let index = self.range.index_of(uid)?;
        let display_index = index as u64 + self.display_base;
        Some(format!("{}{}{}", self.prefix, display_index, self.suffix))
    }
}

pub struct SortedMap<'a, K, V> {
    pub items: &'a [(K, V)],
}

impl<'a, K, V> SortedMap<'a, K, V> {
    pub fn get(&self, key: &K) -> Option<&V>
    where
        K: Ord + PartialEq,
    {
        self.items.binary_search_by_key(&key, |(k, _)| k).ok().map(|index| &self.items[index].1)
    }
}

pub struct TableIdLookup<'a> {
    pub by_name: SortedMap<'a, &'a str, TableRef>,
//L-----------------------------------------------------------------------------
//L Copyright (C) Péter Kardos
//L Please refer to the full license distributed with this software.
//L-----------------------------------------------------------------------------

#[repr(transparent)]
pub struct Path {
    path: str,
}

impl Path {
    pub fn new(s: &str) -> &Self {
        unsafe { &*(s as *const str as *const Path) }
    }

    pub fn security_provider(&self) -> Option<&str> {
        let first_sep = self.path.rfind("::")?;
        let second_sep = self.path[..first_sep].rfind("::")?;
        Some(&self.path[..second_sep])
    }

    pub fn table(&self) -> Option<&str> {
        let mut it = self.path.rsplit("::");
        it.next()?;
        it.next()
    }

    pub fn object(&self) -> &str {
        self.path.rsplit("::").next().unwrap_or("")
    }
}

impl AsRef<Path> for str {
    fn as_ref(&self) -> &Path {
        Path::new(self)
    }
}

impl AsRef<Path> for String {
    fn as_ref(&self) -> &Path {
        Path::new(self)
    }
}

impl AsRef<Path> for Path {
    fn as_ref(&self) -> &Path {
        self
    }
}

impl From<&Path> for String {
    fn from(value: &Path) -> Self {
        value.path.into()
    }
}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.path)
    }
//L-----------------------------------------------------------------------------
//L Copyright (C) Péter Kardos
//L Please refer to the full license distributed with this software.
//L-----------------------------------------------------------------------------

use std::any::Any;
use std::collections::BTreeMap;

use sed_packet::Object;
use sed_spec::objects::{
    AccessControl, Ace, Authority, CPin, KAes256, LockingInfoExt, LockingRange, MbrControl,
    SecurityProvider as SecurityProviderObj, TableDesc,
};

pub type Table<T> = BTreeMap<<T as Object>::Ref, T>;

pub trait SecurityProvider {
    // Tables all SPs have.
    fn access_control(&self) -> &Table<AccessControl>;
    fn access_control_mut(&mut self) -> &mut Table<AccessControl>;
    fn ace(&self) -> &Table<Ace>;
    fn ace_mut(&mut self) -> &mut Table<Ace>;
    fn authority(&self) -> &Table<Authority>;
    fn authority_mut(&mut self) -> &mut Table<Authority>;
    fn c_pin(&self) -> &Table<CPin>;
    fn c_pin_mut(&mut self) -> &mut Table<CPin>;
    fn table(&self) -> &Table<TableDesc>;
    fn table_mut(&mut self) -> &mut Table<TableDesc>;

    // Tables some SPs have.
    fn k_aes_256(&self) -> Option<&Table<KAes256>> {
        None
    }
    fn k_aes_256_mut(&mut self) -> Option<&mut Table<KAes256>> {
        None
    }
    fn locking(&self) -> Option<&Table<LockingRange>> {
        None
    }
    fn locking_mut(&mut self) -> Option<&mut Table<LockingRange>> {
        None
    }
    fn locking_info_ext(&self) -> Option<&Table<LockingInfoExt>> {
        None
    }
    fn locking_info_ext_mut(&mut self) -> Option<&mut Table<LockingInfoExt>> {
        None
    }
    fn mbr_control(&self) -> Option<&Table<MbrControl>> {
        None
    }
    fn mbr_control_mut(&mut self) -> Option<&mut Table<MbrControl>> {
        None
    }
    fn sp(&self) -> Option<&Table<SecurityProviderObj>> {
        None
    }
    fn sp_mut(&mut self) -> Option<&mut Table<SecurityProviderObj>> {
        None
    }
    fn mbr(&self) -> Option<&Vec<u8>> {
        None
    }
    fn mbr_mut(&mut self) -> Option<&mut Vec<u8>> {
        None
    }
    fn data_store(&self, index: usize) -> Option<&Vec<u8>> {
        let _ = index;
        None
    }
    fn data_store_mut(&mut self, index: usize) -> Option<&mut Vec<u8>> {
        let _ = index;
        None
    }

    // Type-erased SPs.
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
f38b4f6 test plan
d038a87 add instrumentation to ACE loading fns
2abc5d5 delete obsolete helpers from locking config session
d8a645b track both k_aes tables for GenKey
e315fad add placeholder test for ACE load
360991b small cleanups, allow MBR for Enterprise
5ef9d0e add load and source functions for remaining ACEs - to review
55cb9a3 load ACEs for authorities
9db5ef6 implement load for authority aces (half-untested)
9a412b8 split access control modules, clean up test
 Cargo.lock                                         |   6 +-
 Cargo.toml                                         |   1 +
 sed_manager/Cargo.toml                             |   1 +
 sed_manager/docs/access_control.md                 | 171 ++++
 sed_manager/docs/access_control_load_tests.md      | 176 +++++
 sed_manager/src/lib.rs                             |   6 +-
 .../src/locking_config/access_control/ace_expr.rs  |  50 ++
 .../src/locking_config/access_control/aces.rs      | 869 +++++++++++++++++++++
 .../access_control/authenticated_set.rs            |  99 +++
 .../src/locking_config/access_control/mod.rs       |  12 +
 .../locking_config/access_control/permissions.rs   | 381 +++++++++
 .../src/locking_config/access_control_plan.rs      |   6 +
 .../src/locking_config/locking_config_session.rs   |  20 +-
 sed_manager/src/locking_config/mod.rs              |   2 +
 sed_manager/src/ssc.rs                             |  34 +
 .../integration/locking_config/access_control.rs   |   5 +
 .../{ => locking_config}/locking_config_session.rs |   0
 .../tests/integration/locking_config/mod.rs        |   2 +
 sed_manager/tests/integration/main.rs              |   2 +-
 sed_packet/src/data_model/uid_range.rs             |   2 +-
 sed_spec/spec.json                                 |   8 +-
 sed_spec/src/objects/ace.rs                        | 296 ++++---
 sed_spec/src/objects/authority.rs                  | 236 ++++++
 sed_spec/src/objects/k_aes_128.rs                  |  31 +
 sed_spec/src/objects/mod.rs                        |   8 +-
 sed_spec_codegen/Cargo.toml                        |   1 +
 sed_spec_codegen/src/lib.rs                        |  85 ++
 sed_virtual_device/src/device.rs                   |  16 +-
 sed_virtual_device/src/session.rs                  |   4 +-
 29 files changed, 2392 insertions(+), 138 deletions(-)
```


**Tool Call: cd /workspaces/sed_manager; git show backend-access-control:sed_manager/src/locking_config/access_control/aces.rs | sed -n 1,120p; grep -n "pub fn\|pub struct\|pub trait" sed_spec/src/lookup.rs sed_spec/src/preconfig.rs | head -30; grep -rn "TokenizeStruct\|DetokenizeStruct" sed_spec_macros/src/lib.rs | head**
Status: Completed

Terminal:
```
//L-----------------------------------------------------------------------------
//L Copyright (C) Péter Kardos
//L Please refer to the full license distributed with this software.
//L-----------------------------------------------------------------------------

use std::collections::BTreeMap;

use sed_packet::UidRange;
use sed_spec::ace_expr;
use sed_spec::methods::MethodStatus;
use sed_spec::objects::{
    AceRef, AceRefExt, AuthorityKind, AuthorityRef, KAes128Ref, KAes128RefExt, KAes256Ref, KAes256RefExt,
    LockingRangeRef,
};
use sed_spec::preconfig::core::shared::table_id;
use sed_spec::preconfig::{core, enterprise, opal_2};
use sed_spec::types::AceOperand;
use sed_tper::{Error as TperError, Session};
use tracing::instrument;

use crate::Error;
use crate::locking_config::access_control::ace_expr::AceExpr;
use crate::ssc::Ssc;

/// The access control settings of the device.
///
/// This structure only contains the part that is important to configure, and is
/// not meant to be an exhaustive list.
///
/// For SSCs that do not allow setting a certain ACE, [`Fixed`] expressions which
/// reflect the preconfiguration are returned. When the ACE can not be read due
/// to a lack of permission, [`Assumed`] values are filled in, which are also
/// equal to the preconfiguration.
///
/// [`Fixed`]: AceExpr::Fixed
/// [`Assumed`]: AceExpr::Assumed
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Aces {
    pub global: GlobalAces,
    pub mbr: Option<MbrAces>,
    pub locking_ranges: BTreeMap<LockingRangeRef, LockingRangeAces>,
    pub authorities: BTreeMap<AuthorityRef, AuthorityAces>,
}

impl Aces {
    #[instrument(skip(session), ret, err)]
    pub async fn load(session: &Session, ssc: Ssc) -> Result<Self, Error> {
        let key_table_support = KeyTableSupport::query(session, ssc).await?;
        let locking_ranges_refs = session.next(None, None).await?;
        let mut locking_ranges = BTreeMap::new();
        for uid in locking_ranges_refs {
            locking_ranges.insert(uid, LockingRangeAces::load(session, uid, ssc, key_table_support).await?);
        }

        let authorities_refs = session.next(None, None).await?;
        let mut authorities = BTreeMap::new();
        for uid in authorities_refs {
            authorities.insert(uid, AuthorityAces::load(session, uid, ssc).await?);
        }

        Ok(Self {
            global: GlobalAces::load(session, ssc).await?,
            mbr: MbrAces::load(session, ssc).await?,
            locking_ranges,
            authorities,
        })
    }
}

/// ACE expressions that are globally important, i.e. not tied to a single
/// authority or locking range.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GlobalAces {
    pub read_ace_table: AceExpr,
    pub set_ace_expressions: AceExpr,
    pub read_authority_table: AceExpr,
    pub read_data_store: AceExpr,
    pub write_data_store: AceExpr,
}

impl GlobalAces {
    #[instrument(skip(session), ret, err)]
    pub async fn load(session: &Session, ssc: Ssc) -> Result<Self, Error> {
        Ok(Self {
            read_ace_table: Self::read_ace_table_source(ssc).load(session).await?,
            set_ace_expressions: Self::set_ace_expressions_source(ssc).load(session).await?,
            read_authority_table: Self::read_authority_table_source(ssc).load(session).await?,
            read_data_store: Self::read_data_store_source(ssc).load(session).await?,
            write_data_store: Self::write_data_store_source(ssc).load(session).await?,
        })
    }

    fn read_ace_table_source(ssc: Ssc) -> AceSource {
        use opal_2::locking::ace::ACE_GET_ALL;
        use opal_2::locking::authority::ADMINS;

        match ssc {
            Ssc::Enterprise => AceSource::Fixed(ace_expr!()),
            _ => AceSource::Editable { ace: ACE_GET_ALL, default: ace_expr!(ADMINS) },
        }
    }

    fn set_ace_expressions_source(ssc: Ssc) -> AceSource {
        use opal_2::locking::authority::ADMINS;

        match ssc {
            Ssc::Enterprise => AceSource::Fixed(ace_expr!()),
            _ => AceSource::Fixed(ace_expr!(ADMINS)),
        }
    }

    fn read_authority_table_source(ssc: Ssc) -> AceSource {
        use enterprise::locking::authority::{BAND_MASTERS, ERASE_MASTER};
        use opal_2::locking::ace::AUTHORITY_GET_ALL;
        use opal_2::locking::authority::ADMINS;

        match ssc {
            // `AnyMaster` is needed to call `Next` on the authority table. The
            // authority objects themselves are only readable by themselves.
            Ssc::Enterprise => AceSource::Fixed(ace_expr!(BAND_MASTERS ERASE_MASTER ||)),
sed_spec/src/lookup.rs:21:pub struct UidRangeEntry<U> {
sed_spec/src/lookup.rs:32:    pub fn by_name(&self, name: &str) -> Option<U> {
sed_spec/src/lookup.rs:39:    pub fn by_uid(&self, uid: U) -> Option<String> {
sed_spec/src/lookup.rs:46:pub struct SortedMap<'a, K, V> {
sed_spec/src/lookup.rs:51:    pub fn get(&self, key: &K) -> Option<&V>
sed_spec/src/lookup.rs:59:pub struct TableIdLookup<'a> {
sed_spec/src/lookup.rs:66:    pub fn by_name(&self, name: &str) -> Option<TableRef> {
sed_spec/src/lookup.rs:73:    pub fn by_uid(&self, uid: TableRef) -> Option<String> {
sed_spec/src/lookup.rs:81:pub trait TableLookup {
sed_spec/src/lookup.rs:88:pub struct ObjectTableLookup<'a, const TABLE: u64> {
sed_spec/src/lookup.rs:127:pub struct MetaTableLookup<'a> {
sed_spec/src/lookup.rs:156:pub struct SecurityProviderLookup<'a> {
sed_spec/src/lookup.rs:163:    pub fn by_path(&self, path: impl AsRef<Path>) -> Option<Uid> {
sed_spec/src/lookup.rs:174:    pub fn by_path_with_table(&self, name: &str, table: TableRef) -> Option<Uid> {
sed_spec/src/lookup.rs:192:    pub fn by_uid(&self, uid: Uid) -> Option<String> {
sed_spec/src/lookup.rs:210:pub struct FeatureLookup<'a> {
sed_spec/src/lookup.rs:215:    pub fn by_path(&self, path: impl AsRef<Path>) -> Option<Uid> {
sed_spec/src/lookup.rs:232:    pub fn by_path_with_sp(&self, path: impl AsRef<Path>, sp: Option<SpRef>) -> Option<Uid> {
sed_spec/src/lookup.rs:237:    pub fn by_path_with_sp_and_table(&self, name: &str, sp: Option<SpRef>, table: TableRef) -> Option<Uid> {
sed_spec/src/lookup.rs:242:    pub fn by_uid(&self, uid: Uid, sp: Option<SpRef>) -> Option<String> {
sed_spec/src/lookup.rs:254:pub struct GlobalLookup;
sed_spec/src/lookup.rs:257:    pub fn by_path(&self, features: &[FeatureDescriptor], path: impl AsRef<Path>) -> Option<Uid> {
sed_spec/src/lookup.rs:262:    pub fn by_path_with_sp(
sed_spec/src/lookup.rs:272:    pub fn by_path_with_sp_and_table(
sed_spec/src/lookup.rs:282:    pub fn by_uid(&self, features: &[FeatureDescriptor], uid: Uid, sp: Option<SpRef>) -> Option<String> {
sed_spec_macros/src/lib.rs:50:#[proc_macro_derive(TokenizeStruct)]
sed_spec_macros/src/lib.rs:62:#[proc_macro_derive(DetokenizeStruct)]
```


**Tool Call: cd /workspaces/sed_manager; grep -rn "pub trait Object\b\|pub trait Object \|pub trait FieldList\|pub trait ObjectUid" -A12 sed_packet/src | head -60; sed -n 1,40p sed_spec/src/objects/access_control.rs | grep -n "pub\|derive"; grep -n "pub" sed_spec/src/objects/authority.rs | head -20**
Status: Completed

Terminal:
```
sed_packet/src/data_model/object_ref.rs:18:pub trait Object {
sed_packet/src/data_model/object_ref.rs-19-    const TABLE: TableRef;
sed_packet/src/data_model/object_ref.rs-20-    type Ref;
sed_packet/src/data_model/object_ref.rs-21-    const FIELD_COUNT: u16;
sed_packet/src/data_model/object_ref.rs-22-
sed_packet/src/data_model/object_ref.rs-23-    fn active_fields(&self) -> Vec<u16>;
sed_packet/src/data_model/object_ref.rs-24-    fn update(&mut self, other: Self);
sed_packet/src/data_model/object_ref.rs-25-    fn field_count(&self) -> u16 {
sed_packet/src/data_model/object_ref.rs-26-        Self::FIELD_COUNT
sed_packet/src/data_model/object_ref.rs-27-    }
sed_packet/src/data_model/object_ref.rs-28-}
sed_packet/src/data_model/object_ref.rs-29-
sed_packet/src/data_model/object_ref.rs-30-pub trait Field<const INDEX: u16> {
--
sed_packet/src/data_model/object_ref.rs:34:pub trait ObjectUid: Object {
sed_packet/src/data_model/object_ref.rs-35-    fn uid(&self) -> Option<Self::Ref>;
sed_packet/src/data_model/object_ref.rs-36-    fn set_uid(&mut self, uid: Option<Self::Ref>);
sed_packet/src/data_model/object_ref.rs-37-}
sed_packet/src/data_model/object_ref.rs-38-
sed_packet/src/data_model/object_ref.rs-39-pub trait TokenizeField: Tokenize {
sed_packet/src/data_model/object_ref.rs-40-    fn tokenize_field<T: Tokenizer>(&self, field: u16, tokenizer: &mut T) -> Result<(), T::Error>;
sed_packet/src/data_model/object_ref.rs-41-}
sed_packet/src/data_model/object_ref.rs-42-
sed_packet/src/data_model/object_ref.rs-43-//------------------------------------------------------------------------------
sed_packet/src/data_model/object_ref.rs-44-// Object reference
sed_packet/src/data_model/object_ref.rs-45-//------------------------------------------------------------------------------
sed_packet/src/data_model/object_ref.rs-46-
12:#[derive(Debug)]
13:pub struct AccessControl {
14:    pub common_name: String,
15:    pub acl: Vec<AceRef>,
16:    pub log: LogSelect,
17:    pub add_ace_acl: Vec<AceRef>,
18:    pub remove_ace_acl: Vec<AceRef>,
19:    pub get_acl_acl: Vec<AceRef>,
20:    pub delete_method_acl: Vec<AceRef>,
21:    pub add_ace_log: LogSelect,
22:    pub remove_ace_log: LogSelect,
23:    pub get_acl_log: LogSelect,
24:    pub delete_method_log: LogSelect,
25:    pub log_to: LogListRef,
15:pub struct Authority {
16:    pub uid: Option<AuthorityRef>,
17:    pub name: Option<String>,
18:    pub common_name: Option<String>,
19:    pub is_class: Option<bool>,
20:    pub class: Option<AuthorityRef>,
21:    pub enabled: Option<bool>,
22:    pub secure: Option<MessagingType>,
23:    pub hash_and_sign: Option<HashProtocol>,
24:    pub present_certificate: Option<bool>,
25:    pub operation: Option<AuthMethod>,
26:    pub credential: Option<Uid>,
27:    pub response_sign: Option<AuthorityRef>,
28:    pub response_exch: Option<AuthorityRef>,
29:    pub clock_start: Option<Date>,
30:    pub clock_end: Option<Date>,
31:    pub limit: Option<u32>,
32:    pub uses: Option<u32>,
33:    pub log: Option<LogSelect>,
34:    pub log_to: Option<LogListRef>,
```


## Assistant

I looked at `sed_spec/src/lookup.rs` (it already does name↔UID lookups at runtime, by path, scoped to an SP and SSC), the object derives, the virtual device's `Table<T>`, and `aces.rs` on `backend-access-control`. Those settle most of the trade-offs.

## Key constraint: names depend on context

A name like `"Admin1"` only becomes a UID once you know the **SSC, SP and table**. Some columns supply the table themselves (`AuthorityRef`, `AceRef`, `CPinRef`), but others don't:
- `Authority::credential` is a bare `Uid`.
- `AccessControlRef::invoking_id` can point at a table, an object or `ThisSP`.
- `Ace::columns` holds column numbers. They could be shown by name if the derives exposed field names.

Serde is stateless, so a derived `Deserialize` can't resolve names unaided. Each approach below handles this differently.

## Approaches

### 1. Build-time codegen (same pattern as `spec.json`)
Write `preconfig/opal_2.json`, and have `build.rs` generate the Rust functions you currently write by hand. Names become references to `sed_spec::preconfig::…` constants.

- **Pros:** the compiler catches unknown names, nothing is parsed at runtime, it suits no_std/EFI, and you already have the tooling.
- **Cons:**
  - The generator has to know each column's type to emit the right expression (e.g. `"Admins"` as a string vs. an authority ref). That means a type-aware generator, or emitting `field: Conv::conv("…")` and letting trait dispatch pick the conversion.
  - It only deserializes, so it does nothing for use case 3 (saving state).
- **Verdict:** covers use cases 1 and 2. You'd still need something else for state saving.

### 2. Runtime serde with name-resolving refs
Derive (or have `sed_spec_macros` generate) `Serialize`/`Deserialize` on the objects. Refs serialize to a name and fall back to a raw UID when no name exists:

```json
{
  "SP": "Locking",
  "Authority": [
    { "uid": "User{n}", "n": "1..=8", "name": "User{n}", "enabled": false,
      "is_class": false, "class": "Users", "operation": "Password",
      "credential": "C_PIN::User{n}" }
  ],
  "ACE": [
    { "uid": "C_PIN_Admins_Set_PIN", "boolean_expr": ["Admins"], "columns": ["PIN"] }
  ],
  "AccessControl": [
    { "invoking_id": "C_PIN::User{n}", "n": "1..=8", "method_id": "Set", "acl": ["C_PIN_User{n}_Set_PIN"] }
  ]
}
```

Options for supplying the SSC/SP context:
- **Thread-local or scoped lookup context:** set it before calling `serde_json::from_str`, and the ref types' impls read it. This is the pragmatic choice and works with `#[derive]`.
- **`DeserializeSeed`:** cleaner, but derive doesn't support it, so you'd have to generate it in `sed_spec_macros`.
- **Two-phase:** deserialize into `NameOrUid` strings, then run a `resolve(&lookup)` pass. This is explicit and testable, but it duplicates every object type (or needs a generic ref parameter).

The UID fallback (`"0x0000000900030001"`) is required for the state-saving round trip, because rows created at runtime and vendor UIDs have no names.

- **Pros:** one format covers all three use cases, it's readable, and `sed_manager` can embed it with `include_str!` for the "assumed/fixed ACE" defaults.
- **Cons:**
  - Unknown names only fail at runtime. A unit test that loads every preconfig would catch them.
  - It adds a serde dependency to `sed_spec`'s object types. serde_json supports `no_std` + `alloc`, so EFI isn't blocked.

### 3. JSON ↔ generic token `Value`, then the existing `Detokenize`
Write a single generic bridge between JSON and `sed_packet`'s token values, and let the existing `TokenizeStruct`/`DetokenizeStruct` derives handle every object type.

- **Pros:** the least per-type work.
- **Cons:**
  - Tokens are untyped, so `"Admins"` (bytes) and an authority UID (also bytes) look the same. You'd need markers in the JSON (`"@Admins"`), or a column-type schema exposed by the derives.
  - Going back to JSON, an 8-byte bytes value can only be guessed to be a UID.
- **Verdict:** fine for authoring, but serializing is lossy or heuristic. Weaker than option 2 unless the derives expose column types.

### 4. Plain tokenized binary
Run `Tokenize` over every table into a byte blob.

- **Pros:** almost free to add, and it round-trips exactly, since it's the same encoding the device speaks.
- **Cons:** not human-readable and painful to diff, so it's wrong for authoring preconfigs.
- **Verdict:** a reasonable cheap first step for saving virtual device state, independent of the authoring format.

### 5. A Rust DSL (`macro_rules!` or a proc-macro)
For example `preconfig! { Authority { User{1..=8}: class=Users, … } }`.

- **Pros:** less verbose and still checked at compile time.
- **Cons:** neither data nor serializable. I'd only consider it if you drop use case 3.

## Cross-cutting design points
- **Repetition:** reuse `spec.json`'s `{n}` convention, but take the count from a parameter (number of ranges/users depends on the device config), not from the file.
- **SSC overlap:** Pyrite and Opalite are mostly subsets of Opal, and Ruby is close to Opal. An `"extends": "Opal_2"` plus add/remove/override overlays would avoid copy-pasting. It also makes the differences explicit, which helps the permissions code.
- **Column names:** have `FieldList` (or a new derive) expose `const FIELD_NAMES: &[&str]`. Then `"columns": ["PIN"]` and JSON object keys resolve generically.
- **Sensitive data:** saved state contains PINs, keys and DataStore contents. That's acceptable for a virtual device, but the (de)serialization path must not log values, and the ref types' `Debug`/tracing should stay as they are.
- **Where it lives:** use cases 1 and 2 both need the preconfig, so the files and loader belong in `sed_spec` (or a new `sed_spec_preconfig` crate), not in `sed_virtual_device`.

## Recommendation
Go with **option 2**, using two-phase or thread-local name resolution with a UID fallback, plus `{n}` templates and SSC overlays. It's the only option that covers authoring, the `sed_manager` defaults and state saving in one readable format, and `lookup.rs` already does most of the hard work. If you want state saving before the JSON work is done, add **option 4** as an interim snapshot format; it costs very little.

I'd skip build-time codegen (option 1) unless no_std/EFI ever needs the preconfig without an allocator. Nothing in the three use cases suggests it will.

## User

We can eventually lift some requirements. `{n}` expansion/interpolation is not necessary in the sense that it's unlikely the virtual device would need to be configured with varying number of objects. `alloc` would also be required even on `no_std` targets, as implementing this application without dynamic memory allocation would be way too complex, and EFI support allocation just fine.

**Tool Call: cd /workspaces/sed_manager; sed -n 562,576p sed_spec/spec.json; sed -n 645,676p sed_spec/spec.json; sed -n 23,27p sed_virtual_device/src/tper/opal_2/preconfig_locking.rs**
Status: Completed

Terminal:
```
                }
            },
            "C_PIN": {
                "SID": "0000000B00000001",
                "MSID": "0000000B00008402",
                "Admin{n}": {
                    "base": "0000000B00000201",
                    "count": 256,
                    "display_base": 1
                }
            },
            "SP": {
                "Admin": "0000020500000001",
                "Locking": "0000020500000002"
            }
                },
                "Users": "0000000900030000",
                "User{n}": {
                    "base": "0000000900030001",
                    "count": 2048,
                    "display_base": 1
                }
            },
            "C_PIN": {
                "Admin{n}": {
                    "base": "0000000B00010001",
                    "count": 2048,
                    "display_base": 1
                },
                "User{n}": {
                    "base": "0000000B00030001",
                    "count": 2048,
                    "display_base": 1
                }
            },
            "Locking": {
                "GlobalRange": "0000080200000001",
                "Range{n}": {
                    "base": "0000080200030001",
                    "count": 2048,
                    "display_base": 1
                }
            },
            "K_AES_128": {
                "GlobalRange_Key": "0000080500000001",
                "Range{n}_Key": {
                    "base": "0000080500030001",
const ADMINS: Range<usize> = 0..4;
const USERS: Range<usize> = 0..8;
const RANGES: Range<usize> = 0..8;
const MBR_SIZE: u32 = 0x08000000;
const DATA_STORE_SIZE: u32 = 0x00A00000;
```


## Assistant

These two changes shift the recommendation further toward option 2, with simpler files.

## Dropping `{n}` expansion

**The files become flat, one row per object.** That's verbose (the Opal Locking SP has 8 ranges × several ACEs each), but each table can be checked line by line against the SSC spec tables, and you no longer need template syntax, a template parser, or rules for how templates interact with overlays.

**The preconfig and saved state share one format.** If the preconfig is just a list of rows, serializing a freshly created virtual device should reproduce the preconfig file exactly. That gives you:
- one schema and one code path for use cases 1 and 3;
- a strong round-trip test: load → serialize → compare, which catches name-resolution and UID-fallback bugs;
- the option of generating the first version of each file by serializing today's hand-written Rust preconfig, then deleting the Rust.

**SSC overlays (`extends`) become optional.** Without templates there's less reason to share structure across SSCs. I'd start with one complete file per SSC. Duplication between Opal, Pyrite and Ruby is tolerable, and each file then matches its own spec document. You can add overlays later if keeping them in sync becomes a burden.

## `alloc` everywhere

With allocation always available, build-time codegen (option 1) has no advantage left except compile-time checking of names, and a unit test that loads every preconfig replaces that. Runtime serde with `include_str!` works on every target. I'd drop option 1.

## Caveat for use case 2 (permission handling)

Flat files suit the virtual device, because you choose its object counts (4 admins, 8 users, 8 ranges in `preconfig_locking.rs:23-25`). Real devices are different. `spec.json` allows up to 256 Admin C_PINs in the Opal Admin SP and 2048 users and ranges in the Locking SP, and vendors ship anywhere from the minimum upward. If `sed_manager` looks up "the preconfigured ACE for `Range12`" in a file listing 8 ranges, it gets nothing.

Options:
- **Use the existing UID ranges to normalize the lookup.** Map `Range12`'s ACEs to their index in the `UidRange`, look up the `Range1` entry, and re-target its UIDs to the same index. This is `{n}` handling again, but done by the code reading the file, using the range data `lookup.rs` already has.
- **Keep the permission-handling defaults in code** (as `aces.rs` on `backend-access-control` already does via `AceSource`), and use the data files only for the virtual device. The defaults there are per kind of object (global, per range, per authority), not per row, which is a different shape from a table dump anyway.
- **List the maximum count in the files.** This is impractical at 2048 rows per table.

I'd take the second option for now. It keeps the data format a plain table dump with no special cases, and use case 2 can move onto the data files later with index normalization if keeping the code in sync with the files becomes painful.

## Revised recommendation

- **Format:** one flat JSON file per SSC and SP. Every row is listed, with names for refs, column names for keys and `columns` entries, and a raw UID fallback.
- **Name resolution:** a scoped (SSC, SP) lookup context during (de)serialization, built on `lookup.rs`.
- **Verification:** a round-trip test (preconfig → device → serialize == preconfig) plus a load-all test.
- **Tokenized binary snapshots:** now probably unnecessary, because the JSON path already gives you state saving.

## User

There is another thing we can relax. For authoring the preconfigurations, it should be human-readable, but for snapshotting the state of a live virtual device, the snapshot can be binary.

## Assistant

With snapshots allowed to be binary, the two concerns separate: JSON is only read, and only for authoring preconfigs, while snapshots use the token encoding you already have.

## Preconfig: JSON, read-only

- **Name resolution only goes one way (name → UID).** Nothing needs a reverse lookup, so the ambiguities that made serializing hard disappear.
- **The UID fallback becomes optional.** Its main purpose was round-tripping rows created at runtime. You might keep it for the odd vendor-specific UID, but it's no longer required.
- **You lose the trick of generating the first JSON files by serializing the current Rust preconfig.** A throwaway script or a one-off `Debug`-based dump can do the migration instead.
- **Option 3 (JSON → generic token values → existing `Detokenize`) is viable again.** Its weakness was lossy serialization, which no longer matters. It still can't tell a string from a ref without type information, so you'd need markers like `"@Admins"` or a column-type schema from the derives. Its advantage is that the existing derives handle every object type.

**Recommendation:** still option 2, but deserialize-only. Use `#[derive(Deserialize)]` on the objects, with custom impls on the ref types and a few special types (`AceOperand`, `MaxBytes`, `columns` resolved by name). Wrap the call in a scoped (SSC, SP) lookup context. Errors point at the right JSON field, and you avoid inventing a marker syntax. Choose option 3 only if adding serde to every object type bothers you more than markers in the JSON.

## Snapshots: tokenized binary

This is close to free. Every object already derives `TokenizeStruct`/`DetokenizeStruct`, and the encoding uses spec-defined column numbers, so old snapshots keep loading after you refactor the Rust structs. You can frame the snapshot in the same token model: a list of SPs, each holding a list of `(table UID, [rows])`.

Things to watch:
- **Large byte arrays.** The MBR is 128 MiB in `preconfig_locking.rs:26`, but the token format's long atom has a 24-bit length, so a single atom tops out just under 16 MiB. Store the MBR and DataStore outside the token stream as raw sections in a simple container, or chunk them. Both are mostly zeros, so sparse storage or compression would shrink snapshots a lot.
- **What counts as state.** Snapshot only state that survives a power cycle: the tables, the MBR and DataStore bytes, and things like the SP life cycle, `Tries` counters and the `Locking` table's lock states. Leave out sessions, ComID state and anything a power cycle resets. That way restoring a snapshot behaves like rebooting a real device, and it doubles as a power-cycle test.
- **A header with a format version and the SSC**, so the loader can reject mismatches instead of misreading them.
- **Sensitive data.** Snapshots hold PINs, keys and DataStore contents. That's fine for a virtual device, but keep snapshot I/O out of traces, as with everything else.

## Testing
The "serialize the preconfig and compare" test is gone. Replace it with:
- **A load-all test:** every preconfig file parses and every name resolves.
- **A snapshot round trip:** load preconfig → snapshot → restore → equal to the original, then the same after a few RPCs that change state (activate, set a PIN, lock a range).
- **A smoke test per SSC:** if `preconfig_admin.rs` and `preconfig_locking.rs` are still around when the JSON lands, compare the JSON-loaded tables with the Rust-built ones once, then delete the Rust files.

## User

What do you think about mixed approaches, like preprocessing the JSON file before deserialization and resolving the object names to UIDs in-place? Names to resolve could be marked like env vars (e.g. `$Admins`), or maybe pattern matching to detect them is enough.

## Assistant

I like it. It solves the context problem more cleanly than anything I suggested before, as long as the names are marked. Pattern matching without markers won't work reliably.

## Why it works well

- **Deserialization becomes stateless.** After preprocessing, every ref is a plain UID, so `AuthorityRef`, `AceRef`, `Uid` and the rest need only a trivial `Deserialize` that parses a hex string (or `u64`). No thread-local, no `DeserializeSeed`, no two-phase types.
- **Raw UIDs come for free.** If the ref types accept hex strings, you can also write `"0x0000000900030001"` directly in the file for vendor UIDs nobody has named.
- **It handles the columns that typed refs couldn't.** `Authority::credential` is a bare `Uid`, and `AccessControlRef::invoking_id` can point at a table, an object or `ThisSP`. A typed-ref approach can't resolve those because the type doesn't say which table the name belongs to. With qualified names in the text (`$C_PIN::SID`, `$Table::C_PIN`, `$ThisSP`), the type doesn't matter.
- **Context comes from the file, not from the code.** One file per (SSC, SP) gives the preprocessor its scope. It prefixes each marked name and passes the path to `lookup.rs`.
- **It's small.** A recursive walk over `serde_json::Value` that rewrites marked strings, then `serde_json::from_value`. Roughly 50 lines plus error handling.

## Why markers are needed

Pattern matching fails in two ways:

- **Names collide with string values.** `Authority { name: "Admins", class: "Admins" }`: the first is a string column and the second is a ref, but they are the same text. Any string column (`name`, `common_name`) can legitimately contain an object's name.
- **Unqualified names collide across tables.** In Opal's Locking SP, `Anybody` exists in both the Authority and the ACE tables with different UIDs, and `GlobalRange` exists in Locking, K_AES_128 and K_AES_256. The preprocessor doesn't know the column type, so the marker must include the table: `$Authority::Anybody` or `$ACE::Anybody`. The extra words cost little and make the file self-explanatory.

Add an escape (`$$` for a literal `$`) so string values can never be misread as names.

## Example

```json
// opal_2/locking.json — SSC and SP come from the file (or a header field)
{
  "Authority": [
    { "uid": "$Authority::User1", "name": "User1", "enabled": false, "is_class": false,
      "class": "$Authority::Users", "operation": "Password", "credential": "$C_PIN::User1" }
  ],
  "ACE": [
    { "uid": "$ACE::C_PIN_Admins_Set_PIN",
      "boolean_expr": ["$Authority::Admins", "$Authority::SID", "||"],
      "columns": [3] }
  ],
  "AccessControl": [
    { "invoking_id": "$C_PIN::User1", "method_id": "$MethodID::Set",
      "acl": ["$ACE::C_PIN_User1_Set_PIN"] }
  ]
}
```

`AceOperand` deserializes from either a UID string (an authority) or `"||"`/`"&&"`/`"!"`.

## Issues to settle

- **Lookup scope.** `$MethodID::Set` and `$ThisSP` live in Core Shared, not in the SP, and `$SP::Locking` lives in the Admin SP. Resolution should try the current SP first, then Core Shared. Allow fully qualified paths (`$Opal_2::Admin::SP::Locking`) for cross-SP references, such as the PSID entries in the Admin SP preconfig.
- **Column names in `columns`.** An ACE's columns belong to the table the ACE protects, which neither the preprocessor nor the type can work out. Either keep them numeric, or use a qualified form like `$C_PIN.PIN`. That would need column names, which neither `spec.json` nor the derives currently provide. Numeric plus a JSON comment is fine to start. Note that standard JSON has no comments: use JSON5, or a `"_comment"` field that is ignored.
- **Error locations.** `from_value` loses line and column numbers. The preprocessor can report unknown names with a JSON pointer (`/ACE/3/boolean_expr/0`), and `serde_path_to_error` gives the same for type errors. That's good enough for hand-authored files covered by a load-all test.
- **Byte values.** PINs and other byte columns need an explicit form, either `{"bytes": "70617373…"}` or a plain string when the bytes are text. The mapping lives in the `Deserialize` impl of `MaxBytes`/`Bytes`, not in the preprocessor.

## Verdict
Use markers with names qualified by table and resolved relative to the file's SSC/SP, plus stateless hex-UID deserialization for the ref types. It's simpler than the scoped-context serde approach, handles the generic `Uid` columns better, and leaves the binary snapshot path unaffected.
