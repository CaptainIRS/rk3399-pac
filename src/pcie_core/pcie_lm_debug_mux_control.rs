#[doc = "Register `PCIE_LM_DEBUG_MUX_CONTROL` reader"]
pub type R = crate::R<PcieLmDebugMuxControlSpec>;
#[doc = "Register `PCIE_LM_DEBUG_MUX_CONTROL` writer"]
pub type W = crate::W<PcieLmDebugMuxControlSpec>;
#[doc = "Field `MS` reader - Mux Select \\[MS\\]\n\nBits 3:2 select the module and bits\n\n1:0 select the group of signals\n\nwithin the module that are driven on\n\nthe debug bus. The assignments of\n\nsignals on the debug outputs of the\n\ncore are given in Appendix B."]
pub type MsR = crate::FieldReader;
#[doc = "Field `MS` writer - Mux Select \\[MS\\]\n\nBits 3:2 select the module and bits\n\n1:0 select the group of signals\n\nwithin the module that are driven on\n\nthe debug bus. The assignments of\n\nsignals on the debug outputs of the\n\ncore are given in Appendix B."]
pub type MsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `R8` reader - Reserved \\[R8\\]\n\n(no description)"]
pub type R8R = crate::FieldReader;
#[doc = "Field `R99` reader - Reserved \\[R99\\]\n\nReserved"]
pub type R99R = crate::BitReader;
#[doc = "Field `R1010` reader - Reserved \\[R1010\\]\n\nReserved"]
pub type R1010R = crate::BitReader;
#[doc = "Field `R1111` reader - Disable Client TX MUX arbitration \\[R1111\\]\n\nWhen this bit is 1, Disable Client TX\n\nMUX Completion and PNP request\n\narbitration, logic added to prevent\n\nPNP requests from starving when\n\ncompletions are present"]
pub type R1111R = crate::BitReader;
#[doc = "Field `R1111` writer - Disable Client TX MUX arbitration \\[R1111\\]\n\nWhen this bit is 1, Disable Client TX\n\nMUX Completion and PNP request\n\narbitration, logic added to prevent\n\nPNP requests from starving when\n\ncompletions are present"]
pub type R1111W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R1212` reader - Reserved \\[R1212\\]\n\n(no description)"]
pub type R1212R = crate::BitReader;
#[doc = "Field `R1313` reader - Reserved \\[R1313\\]\n\n(no description)"]
pub type R1313R = crate::BitReader;
#[doc = "Field `DSSPLM` reader - Disable Set Slot Power Limit Message \\[DSSPLM\\]\n\nDisable sending Set Slot Power Limit\n\nMessage if the Slot Capability\n\nregister is configured"]
pub type DssplmR = crate::BitReader;
#[doc = "Field `DSSPLM` writer - Disable Set Slot Power Limit Message \\[DSSPLM\\]\n\nDisable sending Set Slot Power Limit\n\nMessage if the Slot Capability\n\nregister is configured"]
pub type DssplmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDS` reader - Force Disable Scrambling \\[FDS\\]\n\nDisable Scrambling/Descrambling\n\nin Gen1/Gen2."]
pub type FdsR = crate::BitReader;
#[doc = "Field `FDS` writer - Force Disable Scrambling \\[FDS\\]\n\nDisable Scrambling/Descrambling\n\nin Gen1/Gen2."]
pub type FdsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWRPRI` reader - Enable AXI Bridge Write Priority \\[AWRPRI\\]\n\nWhen this bit is 1, the AXI bridge\n\nplaces a write request on the HAL\n\nMaster interface in preference over\n\na read request if both AXI write and\n\nAXI read requests are available to\n\nbe asserted on the same clock cycle."]
pub type AwrpriR = crate::BitReader;
#[doc = "Field `AWRPRI` writer - Enable AXI Bridge Write Priority \\[AWRPRI\\]\n\nWhen this bit is 1, the AXI bridge\n\nplaces a write request on the HAL\n\nMaster interface in preference over\n\na read request if both AXI write and\n\nAXI read requests are available to\n\nbe asserted on the same clock cycle."]
pub type AwrpriW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R8B` reader - Reserved \\[R8B\\]\n\n(no description)"]
pub type R8bR = crate::BitReader;
#[doc = "Field `R1918` reader - Reserved \\[R1918\\]\n\n(no description)"]
pub type R1918R = crate::FieldReader;
#[doc = "Field `DCIVMC` reader - Disable checking of invalid message codes \\[DCIVMC\\]\n\nWhen this bit is 1, the core will not\n\ncheck for invalid message codes.\n\nThis bit should normally set to 0, as\n\nthe invalid message code checking\n\nis mandatory in the PCIe 3.0\n\nspecifications."]
pub type DcivmcR = crate::BitReader;
#[doc = "Field `DCIVMC` writer - Disable checking of invalid message codes \\[DCIVMC\\]\n\nWhen this bit is 1, the core will not\n\ncheck for invalid message codes.\n\nThis bit should normally set to 0, as\n\nthe invalid message code checking\n\nis mandatory in the PCIe 3.0\n\nspecifications."]
pub type DcivmcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R21` reader - Reserved \\[R21\\]\n\nReserved"]
pub type R21R = crate::FieldReader;
#[doc = "Field `DLUC` reader - Disable Link Upconfigure Capability \\[DLUC\\]\n\nThe user may set this bit to turn off\n\nthe link upconfigure capability of the\n\ncore. Setting this bit prevents the\n\ncore from advertising the link\n\nupconfigure capability in training\n\nsequences transmitted in the\n\nConfiguration.Complete state."]
pub type DlucR = crate::BitReader;
#[doc = "Field `DLUC` writer - Disable Link Upconfigure Capability \\[DLUC\\]\n\nThe user may set this bit to turn off\n\nthe link upconfigure capability of the\n\ncore. Setting this bit prevents the\n\ncore from advertising the link\n\nupconfigure capability in training\n\nsequences transmitted in the\n\nConfiguration.Complete state."]
pub type DlucW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EFLT` reader - Enable Fast Link Training \\[EFLT\\]\n\nThis bit is provided to shorten the\n\nlink training time to facilitate fast\n\nsimulation of the design, especially\n\nat the gate level. Enabling this bit\n\nhas the following effects: 1. The\n\n1ms, 2ms, 12ms, 24ms, 32ms and\n\n48ms timeout intervals in the LTSSM\n\nare shortened by a factor of 500. 2.\n\nIn the Polling.Active state of the\n\nLTSSM, only 16 training sequences\n\nare required to be transmitted\n\n(Instead of 1024) to make the\n\ntransition to the Configuration state.\n\nThis bit should not be set during\n\nnormal operation of the core."]
pub type EfltR = crate::BitReader;
#[doc = "Field `EFLT` writer - Enable Fast Link Training \\[EFLT\\]\n\nThis bit is provided to shorten the\n\nlink training time to facilitate fast\n\nsimulation of the design, especially\n\nat the gate level. Enabling this bit\n\nhas the following effects: 1. The\n\n1ms, 2ms, 12ms, 24ms, 32ms and\n\n48ms timeout intervals in the LTSSM\n\nare shortened by a factor of 500. 2.\n\nIn the Polling.Active state of the\n\nLTSSM, only 16 training sequences\n\nare required to be transmitted\n\n(Instead of 1024) to make the\n\ntransition to the Configuration state.\n\nThis bit should not be set during\n\nnormal operation of the core."]
pub type EfltW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ESPC` reader - Enable Slot Power Capture \\[ESPC\\]\n\nWhen this bit is set to 1, the core\n\nwill capture the Slot Power Limit\n\nValue and Slot Power Limit Scale\n\nparameters from a\n\nSet_Slot_Power_Limit message\n\nreceived in the Device Capabilities\n\nRegister. When this bit is 0, the\n\ncapture is disabled. This bit is valid\n\nonly when the core is configured\n\nas an EndPoint. It has no effect\n\nwhen the core is a Root Complex."]
pub type EspcR = crate::BitReader;
#[doc = "Field `ESPC` writer - Enable Slot Power Capture \\[ESPC\\]\n\nWhen this bit is set to 1, the core\n\nwill capture the Slot Power Limit\n\nValue and Slot Power Limit Scale\n\nparameters from a\n\nSet_Slot_Power_Limit message\n\nreceived in the Device Capabilities\n\nRegister. When this bit is 0, the\n\ncapture is disabled. This bit is valid\n\nonly when the core is configured\n\nas an EndPoint. It has no effect\n\nwhen the core is a Root Complex."]
pub type EspcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R26` reader - Reserved \\[R26\\]\n\nReserved"]
pub type R26R = crate::BitReader;
#[doc = "Field `R27` reader - Reserved \\[R27\\]\n\nReserved"]
pub type R27R = crate::BitReader;
#[doc = "Field `DEI` reader - Disable Electrical Idle Infer in L0 State \\[DEI\\]\n\nSetting this bit to 1 disables the\n\ninferring of electrical idle in the L0\n\nstate. Electrical idle is inferred when\n\nno flow control updates and no SKP\n\nsequences are received within an\n\ninterval of 128 us. This bit should\n\nnot be set during normal operation,\n\nbut is useful for testing."]
pub type DeiR = crate::BitReader;
#[doc = "Field `DEI` writer - Disable Electrical Idle Infer in L0 State \\[DEI\\]\n\nSetting this bit to 1 disables the\n\ninferring of electrical idle in the L0\n\nstate. Electrical idle is inferred when\n\nno flow control updates and no SKP\n\nsequences are received within an\n\ninterval of 128 us. This bit should\n\nnot be set during normal operation,\n\nbut is useful for testing."]
pub type DeiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFCUT` reader - Disable Flow Control Update Timeout \\[DFCUT\\]\n\nWhen this bit is 0, the core will time\n\nout and re-train the link when no\n\nFlow Control Update DLLPs are\n\nreceived from the link within an\n\ninterval of 128 us. Setting this bit to\n\n1 disables\n\nthis timeout. When the advertised\n\nreceive credit of the link partner is\n\ninfinity for the header and payload\n\nof all credit types, this timeout is\n\nalways suppressed. The setting of\n\nthis bit has no effect in this case.\n\nThis bit should not be set during\n\nnormal operation, but is useful for\n\ntesting."]
pub type DfcutR = crate::BitReader;
#[doc = "Field `DFCUT` writer - Disable Flow Control Update Timeout \\[DFCUT\\]\n\nWhen this bit is 0, the core will time\n\nout and re-train the link when no\n\nFlow Control Update DLLPs are\n\nreceived from the link within an\n\ninterval of 128 us. Setting this bit to\n\n1 disables\n\nthis timeout. When the advertised\n\nreceive credit of the link partner is\n\ninfinity for the header and payload\n\nof all credit types, this timeout is\n\nalways suppressed. The setting of\n\nthis bit has no effect in this case.\n\nThis bit should not be set during\n\nnormal operation, but is useful for\n\ntesting."]
pub type DfcutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOC` reader - Disable Ordering Checks \\[DOC\\]\n\nSetting this bit to 1 disables the\n\nordering check in the core between\n\nCompletions and Posted requests\n\nreceived from the link."]
pub type DocR = crate::BitReader;
#[doc = "Field `DOC` writer - Disable Ordering Checks \\[DOC\\]\n\nSetting this bit to 1 disables the\n\nordering check in the core between\n\nCompletions and Posted requests\n\nreceived from the link."]
pub type DocW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EFSRTCA` reader - Enable Function- Specific Reporting of Type-1 Configuration Accesses \\[EFSRTCA\\]\n\nSetting this bit to 0 causes all the\n\nenabled Functions to report an error\n\nwhen a Type-1 configuration access\n\nis received by the core, targeted at\n\nany Function. Setting it to 1 limits\n\nthe error reporting to the type-0\n\nFunction whose number matches\n\nwith the Function number specified\n\nin the request. If the Function\n\nnumber in the request refers to an\n\nunimplemented or disabled\n\nFunction, all enabled Functions\n\nreport the error regardless of the\n\nsetting of this bit."]
pub type EfsrtcaR = crate::BitReader;
#[doc = "Field `EFSRTCA` writer - Enable Function- Specific Reporting of Type-1 Configuration Accesses \\[EFSRTCA\\]\n\nSetting this bit to 0 causes all the\n\nenabled Functions to report an error\n\nwhen a Type-1 configuration access\n\nis received by the core, targeted at\n\nany Function. Setting it to 1 limits\n\nthe error reporting to the type-0\n\nFunction whose number matches\n\nwith the Function number specified\n\nin the request. If the Function\n\nnumber in the request refers to an\n\nunimplemented or disabled\n\nFunction, all enabled Functions\n\nreport the error regardless of the\n\nsetting of this bit."]
pub type EfsrtcaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Mux Select \\[MS\\]\n\nBits 3:2 select the module and bits\n\n1:0 select the group of signals\n\nwithin the module that are driven on\n\nthe debug bus. The assignments of\n\nsignals on the debug outputs of the\n\ncore are given in Appendix B."]
    #[inline(always)]
    pub fn ms(&self) -> MsR {
        MsR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:8 - Reserved \\[R8\\]\n\n(no description)"]
    #[inline(always)]
    pub fn r8(&self) -> R8R {
        R8R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bit 9 - Reserved \\[R99\\]\n\nReserved"]
    #[inline(always)]
    pub fn r99(&self) -> R99R {
        R99R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Reserved \\[R1010\\]\n\nReserved"]
    #[inline(always)]
    pub fn r1010(&self) -> R1010R {
        R1010R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Disable Client TX MUX arbitration \\[R1111\\]\n\nWhen this bit is 1, Disable Client TX\n\nMUX Completion and PNP request\n\narbitration, logic added to prevent\n\nPNP requests from starving when\n\ncompletions are present"]
    #[inline(always)]
    pub fn r1111(&self) -> R1111R {
        R1111R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Reserved \\[R1212\\]\n\n(no description)"]
    #[inline(always)]
    pub fn r1212(&self) -> R1212R {
        R1212R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Reserved \\[R1313\\]\n\n(no description)"]
    #[inline(always)]
    pub fn r1313(&self) -> R1313R {
        R1313R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Disable Set Slot Power Limit Message \\[DSSPLM\\]\n\nDisable sending Set Slot Power Limit\n\nMessage if the Slot Capability\n\nregister is configured"]
    #[inline(always)]
    pub fn dssplm(&self) -> DssplmR {
        DssplmR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Force Disable Scrambling \\[FDS\\]\n\nDisable Scrambling/Descrambling\n\nin Gen1/Gen2."]
    #[inline(always)]
    pub fn fds(&self) -> FdsR {
        FdsR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable AXI Bridge Write Priority \\[AWRPRI\\]\n\nWhen this bit is 1, the AXI bridge\n\nplaces a write request on the HAL\n\nMaster interface in preference over\n\na read request if both AXI write and\n\nAXI read requests are available to\n\nbe asserted on the same clock cycle."]
    #[inline(always)]
    pub fn awrpri(&self) -> AwrpriR {
        AwrpriR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Reserved \\[R8B\\]\n\n(no description)"]
    #[inline(always)]
    pub fn r8b(&self) -> R8bR {
        R8bR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Reserved \\[R1918\\]\n\n(no description)"]
    #[inline(always)]
    pub fn r1918(&self) -> R1918R {
        R1918R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Disable checking of invalid message codes \\[DCIVMC\\]\n\nWhen this bit is 1, the core will not\n\ncheck for invalid message codes.\n\nThis bit should normally set to 0, as\n\nthe invalid message code checking\n\nis mandatory in the PCIe 3.0\n\nspecifications."]
    #[inline(always)]
    pub fn dcivmc(&self) -> DcivmcR {
        DcivmcR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - Reserved \\[R21\\]\n\nReserved"]
    #[inline(always)]
    pub fn r21(&self) -> R21R {
        R21R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - Disable Link Upconfigure Capability \\[DLUC\\]\n\nThe user may set this bit to turn off\n\nthe link upconfigure capability of the\n\ncore. Setting this bit prevents the\n\ncore from advertising the link\n\nupconfigure capability in training\n\nsequences transmitted in the\n\nConfiguration.Complete state."]
    #[inline(always)]
    pub fn dluc(&self) -> DlucR {
        DlucR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable Fast Link Training \\[EFLT\\]\n\nThis bit is provided to shorten the\n\nlink training time to facilitate fast\n\nsimulation of the design, especially\n\nat the gate level. Enabling this bit\n\nhas the following effects: 1. The\n\n1ms, 2ms, 12ms, 24ms, 32ms and\n\n48ms timeout intervals in the LTSSM\n\nare shortened by a factor of 500. 2.\n\nIn the Polling.Active state of the\n\nLTSSM, only 16 training sequences\n\nare required to be transmitted\n\n(Instead of 1024) to make the\n\ntransition to the Configuration state.\n\nThis bit should not be set during\n\nnormal operation of the core."]
    #[inline(always)]
    pub fn eflt(&self) -> EfltR {
        EfltR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable Slot Power Capture \\[ESPC\\]\n\nWhen this bit is set to 1, the core\n\nwill capture the Slot Power Limit\n\nValue and Slot Power Limit Scale\n\nparameters from a\n\nSet_Slot_Power_Limit message\n\nreceived in the Device Capabilities\n\nRegister. When this bit is 0, the\n\ncapture is disabled. This bit is valid\n\nonly when the core is configured\n\nas an EndPoint. It has no effect\n\nwhen the core is a Root Complex."]
    #[inline(always)]
    pub fn espc(&self) -> EspcR {
        EspcR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Reserved \\[R26\\]\n\nReserved"]
    #[inline(always)]
    pub fn r26(&self) -> R26R {
        R26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Reserved \\[R27\\]\n\nReserved"]
    #[inline(always)]
    pub fn r27(&self) -> R27R {
        R27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Disable Electrical Idle Infer in L0 State \\[DEI\\]\n\nSetting this bit to 1 disables the\n\ninferring of electrical idle in the L0\n\nstate. Electrical idle is inferred when\n\nno flow control updates and no SKP\n\nsequences are received within an\n\ninterval of 128 us. This bit should\n\nnot be set during normal operation,\n\nbut is useful for testing."]
    #[inline(always)]
    pub fn dei(&self) -> DeiR {
        DeiR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Disable Flow Control Update Timeout \\[DFCUT\\]\n\nWhen this bit is 0, the core will time\n\nout and re-train the link when no\n\nFlow Control Update DLLPs are\n\nreceived from the link within an\n\ninterval of 128 us. Setting this bit to\n\n1 disables\n\nthis timeout. When the advertised\n\nreceive credit of the link partner is\n\ninfinity for the header and payload\n\nof all credit types, this timeout is\n\nalways suppressed. The setting of\n\nthis bit has no effect in this case.\n\nThis bit should not be set during\n\nnormal operation, but is useful for\n\ntesting."]
    #[inline(always)]
    pub fn dfcut(&self) -> DfcutR {
        DfcutR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Disable Ordering Checks \\[DOC\\]\n\nSetting this bit to 1 disables the\n\nordering check in the core between\n\nCompletions and Posted requests\n\nreceived from the link."]
    #[inline(always)]
    pub fn doc(&self) -> DocR {
        DocR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Function- Specific Reporting of Type-1 Configuration Accesses \\[EFSRTCA\\]\n\nSetting this bit to 0 causes all the\n\nenabled Functions to report an error\n\nwhen a Type-1 configuration access\n\nis received by the core, targeted at\n\nany Function. Setting it to 1 limits\n\nthe error reporting to the type-0\n\nFunction whose number matches\n\nwith the Function number specified\n\nin the request. If the Function\n\nnumber in the request refers to an\n\nunimplemented or disabled\n\nFunction, all enabled Functions\n\nreport the error regardless of the\n\nsetting of this bit."]
    #[inline(always)]
    pub fn efsrtca(&self) -> EfsrtcaR {
        EfsrtcaR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Mux Select \\[MS\\]\n\nBits 3:2 select the module and bits\n\n1:0 select the group of signals\n\nwithin the module that are driven on\n\nthe debug bus. The assignments of\n\nsignals on the debug outputs of the\n\ncore are given in Appendix B."]
    #[inline(always)]
    #[must_use]
    pub fn ms(&mut self) -> MsW<PcieLmDebugMuxControlSpec> {
        MsW::new(self, 0)
    }
    #[doc = "Bit 11 - Disable Client TX MUX arbitration \\[R1111\\]\n\nWhen this bit is 1, Disable Client TX\n\nMUX Completion and PNP request\n\narbitration, logic added to prevent\n\nPNP requests from starving when\n\ncompletions are present"]
    #[inline(always)]
    #[must_use]
    pub fn r1111(&mut self) -> R1111W<PcieLmDebugMuxControlSpec> {
        R1111W::new(self, 11)
    }
    #[doc = "Bit 14 - Disable Set Slot Power Limit Message \\[DSSPLM\\]\n\nDisable sending Set Slot Power Limit\n\nMessage if the Slot Capability\n\nregister is configured"]
    #[inline(always)]
    #[must_use]
    pub fn dssplm(&mut self) -> DssplmW<PcieLmDebugMuxControlSpec> {
        DssplmW::new(self, 14)
    }
    #[doc = "Bit 15 - Force Disable Scrambling \\[FDS\\]\n\nDisable Scrambling/Descrambling\n\nin Gen1/Gen2."]
    #[inline(always)]
    #[must_use]
    pub fn fds(&mut self) -> FdsW<PcieLmDebugMuxControlSpec> {
        FdsW::new(self, 15)
    }
    #[doc = "Bit 16 - Enable AXI Bridge Write Priority \\[AWRPRI\\]\n\nWhen this bit is 1, the AXI bridge\n\nplaces a write request on the HAL\n\nMaster interface in preference over\n\na read request if both AXI write and\n\nAXI read requests are available to\n\nbe asserted on the same clock cycle."]
    #[inline(always)]
    #[must_use]
    pub fn awrpri(&mut self) -> AwrpriW<PcieLmDebugMuxControlSpec> {
        AwrpriW::new(self, 16)
    }
    #[doc = "Bit 20 - Disable checking of invalid message codes \\[DCIVMC\\]\n\nWhen this bit is 1, the core will not\n\ncheck for invalid message codes.\n\nThis bit should normally set to 0, as\n\nthe invalid message code checking\n\nis mandatory in the PCIe 3.0\n\nspecifications."]
    #[inline(always)]
    #[must_use]
    pub fn dcivmc(&mut self) -> DcivmcW<PcieLmDebugMuxControlSpec> {
        DcivmcW::new(self, 20)
    }
    #[doc = "Bit 23 - Disable Link Upconfigure Capability \\[DLUC\\]\n\nThe user may set this bit to turn off\n\nthe link upconfigure capability of the\n\ncore. Setting this bit prevents the\n\ncore from advertising the link\n\nupconfigure capability in training\n\nsequences transmitted in the\n\nConfiguration.Complete state."]
    #[inline(always)]
    #[must_use]
    pub fn dluc(&mut self) -> DlucW<PcieLmDebugMuxControlSpec> {
        DlucW::new(self, 23)
    }
    #[doc = "Bit 24 - Enable Fast Link Training \\[EFLT\\]\n\nThis bit is provided to shorten the\n\nlink training time to facilitate fast\n\nsimulation of the design, especially\n\nat the gate level. Enabling this bit\n\nhas the following effects: 1. The\n\n1ms, 2ms, 12ms, 24ms, 32ms and\n\n48ms timeout intervals in the LTSSM\n\nare shortened by a factor of 500. 2.\n\nIn the Polling.Active state of the\n\nLTSSM, only 16 training sequences\n\nare required to be transmitted\n\n(Instead of 1024) to make the\n\ntransition to the Configuration state.\n\nThis bit should not be set during\n\nnormal operation of the core."]
    #[inline(always)]
    #[must_use]
    pub fn eflt(&mut self) -> EfltW<PcieLmDebugMuxControlSpec> {
        EfltW::new(self, 24)
    }
    #[doc = "Bit 25 - Enable Slot Power Capture \\[ESPC\\]\n\nWhen this bit is set to 1, the core\n\nwill capture the Slot Power Limit\n\nValue and Slot Power Limit Scale\n\nparameters from a\n\nSet_Slot_Power_Limit message\n\nreceived in the Device Capabilities\n\nRegister. When this bit is 0, the\n\ncapture is disabled. This bit is valid\n\nonly when the core is configured\n\nas an EndPoint. It has no effect\n\nwhen the core is a Root Complex."]
    #[inline(always)]
    #[must_use]
    pub fn espc(&mut self) -> EspcW<PcieLmDebugMuxControlSpec> {
        EspcW::new(self, 25)
    }
    #[doc = "Bit 28 - Disable Electrical Idle Infer in L0 State \\[DEI\\]\n\nSetting this bit to 1 disables the\n\ninferring of electrical idle in the L0\n\nstate. Electrical idle is inferred when\n\nno flow control updates and no SKP\n\nsequences are received within an\n\ninterval of 128 us. This bit should\n\nnot be set during normal operation,\n\nbut is useful for testing."]
    #[inline(always)]
    #[must_use]
    pub fn dei(&mut self) -> DeiW<PcieLmDebugMuxControlSpec> {
        DeiW::new(self, 28)
    }
    #[doc = "Bit 29 - Disable Flow Control Update Timeout \\[DFCUT\\]\n\nWhen this bit is 0, the core will time\n\nout and re-train the link when no\n\nFlow Control Update DLLPs are\n\nreceived from the link within an\n\ninterval of 128 us. Setting this bit to\n\n1 disables\n\nthis timeout. When the advertised\n\nreceive credit of the link partner is\n\ninfinity for the header and payload\n\nof all credit types, this timeout is\n\nalways suppressed. The setting of\n\nthis bit has no effect in this case.\n\nThis bit should not be set during\n\nnormal operation, but is useful for\n\ntesting."]
    #[inline(always)]
    #[must_use]
    pub fn dfcut(&mut self) -> DfcutW<PcieLmDebugMuxControlSpec> {
        DfcutW::new(self, 29)
    }
    #[doc = "Bit 30 - Disable Ordering Checks \\[DOC\\]\n\nSetting this bit to 1 disables the\n\nordering check in the core between\n\nCompletions and Posted requests\n\nreceived from the link."]
    #[inline(always)]
    #[must_use]
    pub fn doc(&mut self) -> DocW<PcieLmDebugMuxControlSpec> {
        DocW::new(self, 30)
    }
    #[doc = "Bit 31 - Enable Function- Specific Reporting of Type-1 Configuration Accesses \\[EFSRTCA\\]\n\nSetting this bit to 0 causes all the\n\nenabled Functions to report an error\n\nwhen a Type-1 configuration access\n\nis received by the core, targeted at\n\nany Function. Setting it to 1 limits\n\nthe error reporting to the type-0\n\nFunction whose number matches\n\nwith the Function number specified\n\nin the request. If the Function\n\nnumber in the request refers to an\n\nunimplemented or disabled\n\nFunction, all enabled Functions\n\nreport the error regardless of the\n\nsetting of this bit."]
    #[inline(always)]
    #[must_use]
    pub fn efsrtca(&mut self) -> EfsrtcaW<PcieLmDebugMuxControlSpec> {
        EfsrtcaW::new(self, 31)
    }
}
#[doc = "Debug Mux Control Register\n\nSetting this bit to 0 causes all the\n\nenabled Functions to report an error\n\nwhen a Type-1 configuration access\n\nis received by the core, targeted at\n\nany Function. Setting it to 1 limits\n\nthe error reporting to the type-0\n\nFunction whose number matches\n\nwith the Function number specified\n\nin the request. If the Function\n\nnumber in the request refers to an\n\nunimplemented or disabled\n\nFunction, all enabled Functions\n\nreport the error regardless of the\n\nsetting of this bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_debug_mux_control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_lm_debug_mux_control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieLmDebugMuxControlSpec;
impl crate::RegisterSpec for PcieLmDebugMuxControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_lm_debug_mux_control::R`](R) reader structure"]
impl crate::Readable for PcieLmDebugMuxControlSpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_lm_debug_mux_control::W`](W) writer structure"]
impl crate::Writable for PcieLmDebugMuxControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_LM_DEBUG_MUX_CONTROL to value 0x8000_0000"]
impl crate::Resettable for PcieLmDebugMuxControlSpec {
    const RESET_VALUE: u32 = 0x8000_0000;
}
