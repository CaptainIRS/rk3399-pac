#[doc = "Register `POWER_CTRL` reader"]
pub type R = crate::R<PowerCtrlSpec>;
#[doc = "Register `POWER_CTRL` writer"]
pub type W = crate::W<PowerCtrlSpec>;
#[doc = "Client request exit L2 power state\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CltReqExitL2 {
    #[doc = "0: keep"]
    B0 = 0,
    #[doc = "1: Exit from L2_IDLE This input can be asserted by the client only in the short interval of time after the link enters L2 and before the system is powered OFF. While the power and clocks are still ON, the client can assert this input to initiate an exit from L2_IDLE->DETECT."]
    B1 = 1,
}
impl From<CltReqExitL2> for bool {
    #[inline(always)]
    fn from(variant: CltReqExitL2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLT_REQ_EXIT_L2` reader - Client request exit L2 power state"]
pub type CltReqExitL2R = crate::BitReader<CltReqExitL2>;
impl CltReqExitL2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CltReqExitL2 {
        match self.bits {
            false => CltReqExitL2::B0,
            true => CltReqExitL2::B1,
        }
    }
    #[doc = "keep"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == CltReqExitL2::B0
    }
    #[doc = "Exit from L2_IDLE This input can be asserted by the client only in the short interval of time after the link enters L2 and before the system is powered OFF. While the power and clocks are still ON, the client can assert this input to initiate an exit from L2_IDLE->DETECT."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == CltReqExitL2::B1
    }
}
#[doc = "Field `CLT_REQ_EXIT_L2` writer - Client request exit L2 power state"]
pub type CltReqExitL2W<'a, REG> = crate::BitWriter<'a, REG, CltReqExitL2>;
impl<'a, REG> CltReqExitL2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "keep"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(CltReqExitL2::B0)
    }
    #[doc = "Exit from L2_IDLE This input can be asserted by the client only in the short interval of time after the link enters L2 and before the system is powered OFF. While the power and clocks are still ON, the client can assert this input to initiate an exit from L2_IDLE->DETECT."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(CltReqExitL2::B1)
    }
}
#[doc = "Request transition to L23_Ready state\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReqTrnL23ready {
    #[doc = "0: keep"]
    B0 = 0,
    #[doc = "1: transition the power management state of the core to L23_READY When the core is configured as Endpoint, the client may assert this input to transition the power management state of the core to L23_READY (see Chapter 5 of PCI Express Specifications for a detailed description of power management). This is done after the PCI Functions in the core have been placed in the D3 state and after the client has acknowledged the PME_Turn_Off message from the Root Port. Asserting this input causes the link to transition to the L2 state, and requires a power-on reset to resume operation. This input can be hardwired to 0 if the link is not required to transition to L2. This input is not used in the Root Port mode."]
    B1 = 1,
}
impl From<ReqTrnL23ready> for bool {
    #[inline(always)]
    fn from(variant: ReqTrnL23ready) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REQ_TRN_L23READY` reader - Request transition to L23_Ready state"]
pub type ReqTrnL23readyR = crate::BitReader<ReqTrnL23ready>;
impl ReqTrnL23readyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ReqTrnL23ready {
        match self.bits {
            false => ReqTrnL23ready::B0,
            true => ReqTrnL23ready::B1,
        }
    }
    #[doc = "keep"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ReqTrnL23ready::B0
    }
    #[doc = "transition the power management state of the core to L23_READY When the core is configured as Endpoint, the client may assert this input to transition the power management state of the core to L23_READY (see Chapter 5 of PCI Express Specifications for a detailed description of power management). This is done after the PCI Functions in the core have been placed in the D3 state and after the client has acknowledged the PME_Turn_Off message from the Root Port. Asserting this input causes the link to transition to the L2 state, and requires a power-on reset to resume operation. This input can be hardwired to 0 if the link is not required to transition to L2. This input is not used in the Root Port mode."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ReqTrnL23ready::B1
    }
}
#[doc = "Field `REQ_TRN_L23READY` writer - Request transition to L23_Ready state"]
pub type ReqTrnL23readyW<'a, REG> = crate::BitWriter<'a, REG, ReqTrnL23ready>;
impl<'a, REG> ReqTrnL23readyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "keep"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ReqTrnL23ready::B0)
    }
    #[doc = "transition the power management state of the core to L23_READY When the core is configured as Endpoint, the client may assert this input to transition the power management state of the core to L23_READY (see Chapter 5 of PCI Express Specifications for a detailed description of power management). This is done after the PCI Functions in the core have been placed in the D3 state and after the client has acknowledged the PME_Turn_Off message from the Root Port. Asserting this input causes the link to transition to the L2 state, and requires a power-on reset to resume operation. This input can be hardwired to 0 if the link is not required to transition to L2. This input is not used in the Root Port mode."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ReqTrnL23ready::B1)
    }
}
#[doc = "Client request exit L1 power state\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CltReqExitL1 {
    #[doc = "0: keep"]
    B0 = 0,
    #[doc = "1: request to exit the L1.1 or L1.2.Idle substate Client request to exit the L1.1 or L1.2.Idle substate. When the core clock is turned off, the client must activate this input to request the L1 PM substate state machine to de-assert CLKREQ_OUT and transition the link out of L1. If the core clock is not turned off in the L1.1 and L1.2 substates, this input can be permanently kept low."]
    B1 = 1,
}
impl From<CltReqExitL1> for bool {
    #[inline(always)]
    fn from(variant: CltReqExitL1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLT_REQ_EXIT_L1` reader - Client request exit L1 power state"]
pub type CltReqExitL1R = crate::BitReader<CltReqExitL1>;
impl CltReqExitL1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CltReqExitL1 {
        match self.bits {
            false => CltReqExitL1::B0,
            true => CltReqExitL1::B1,
        }
    }
    #[doc = "keep"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == CltReqExitL1::B0
    }
    #[doc = "request to exit the L1.1 or L1.2.Idle substate Client request to exit the L1.1 or L1.2.Idle substate. When the core clock is turned off, the client must activate this input to request the L1 PM substate state machine to de-assert CLKREQ_OUT and transition the link out of L1. If the core clock is not turned off in the L1.1 and L1.2 substates, this input can be permanently kept low."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == CltReqExitL1::B1
    }
}
#[doc = "Field `CLT_REQ_EXIT_L1` writer - Client request exit L1 power state"]
pub type CltReqExitL1W<'a, REG> = crate::BitWriter<'a, REG, CltReqExitL1>;
impl<'a, REG> CltReqExitL1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "keep"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(CltReqExitL1::B0)
    }
    #[doc = "request to exit the L1.1 or L1.2.Idle substate Client request to exit the L1.1 or L1.2.Idle substate. When the core clock is turned off, the client must activate this input to request the L1 PM substate state machine to de-assert CLKREQ_OUT and transition the link out of L1. If the core clock is not turned off in the L1.1 and L1.2 substates, this input can be permanently kept low."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(CltReqExitL1::B1)
    }
}
#[doc = "Hardware clear exit L2 request\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HwclrExitL2Req {
    #[doc = "0: software polling and write clear mode"]
    B0 = 0,
    #[doc = "1: hardware polling and auto-clear mode"]
    B1 = 1,
}
impl From<HwclrExitL2Req> for bool {
    #[inline(always)]
    fn from(variant: HwclrExitL2Req) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HWCLR_EXIT_L2_REQ` reader - Hardware clear exit L2 request"]
pub type HwclrExitL2ReqR = crate::BitReader<HwclrExitL2Req>;
impl HwclrExitL2ReqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HwclrExitL2Req {
        match self.bits {
            false => HwclrExitL2Req::B0,
            true => HwclrExitL2Req::B1,
        }
    }
    #[doc = "software polling and write clear mode"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == HwclrExitL2Req::B0
    }
    #[doc = "hardware polling and auto-clear mode"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == HwclrExitL2Req::B1
    }
}
#[doc = "Field `HWCLR_EXIT_L2_REQ` writer - Hardware clear exit L2 request"]
pub type HwclrExitL2ReqW<'a, REG> = crate::BitWriter<'a, REG, HwclrExitL2Req>;
impl<'a, REG> HwclrExitL2ReqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "software polling and write clear mode"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(HwclrExitL2Req::B0)
    }
    #[doc = "hardware polling and auto-clear mode"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(HwclrExitL2Req::B1)
    }
}
#[doc = "Hardware clear exit L1 request\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HwclrExitL1Req {
    #[doc = "0: software polling and write clear mode"]
    B0 = 0,
    #[doc = "1: hardware polling and auto-clear mode"]
    B1 = 1,
}
impl From<HwclrExitL1Req> for bool {
    #[inline(always)]
    fn from(variant: HwclrExitL1Req) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HWCLR_EXIT_L1_REQ` reader - Hardware clear exit L1 request"]
pub type HwclrExitL1ReqR = crate::BitReader<HwclrExitL1Req>;
impl HwclrExitL1ReqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HwclrExitL1Req {
        match self.bits {
            false => HwclrExitL1Req::B0,
            true => HwclrExitL1Req::B1,
        }
    }
    #[doc = "software polling and write clear mode"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == HwclrExitL1Req::B0
    }
    #[doc = "hardware polling and auto-clear mode"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == HwclrExitL1Req::B1
    }
}
#[doc = "Field `HWCLR_EXIT_L1_REQ` writer - Hardware clear exit L1 request"]
pub type HwclrExitL1ReqW<'a, REG> = crate::BitWriter<'a, REG, HwclrExitL1Req>;
impl<'a, REG> HwclrExitL1ReqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "software polling and write clear mode"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(HwclrExitL1Req::B0)
    }
    #[doc = "hardware polling and auto-clear mode"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(HwclrExitL1Req::B1)
    }
}
#[doc = "Power state change ack\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PwrStcgAck {
    #[doc = "0: keep 0"]
    B0 = 0,
    #[doc = "1: write 1 to generate one high pulse ack to controller The client must assert this input to the core for one cycle in response to the assertion of power state change interrupt, when it is ready to transition to the low-power state requested by the configuration write request. The client may permanently maintain this input high if it does not need to delay the return of the completions for the configuration write transactions causing power-state changes."]
    B1 = 1,
}
impl From<PwrStcgAck> for bool {
    #[inline(always)]
    fn from(variant: PwrStcgAck) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWR_STCG_ACK` writer - Power state change ack"]
pub type PwrStcgAckW<'a, REG> = crate::BitWriter<'a, REG, PwrStcgAck>;
impl<'a, REG> PwrStcgAckW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "keep 0"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PwrStcgAck::B0)
    }
    #[doc = "write 1 to generate one high pulse ack to controller The client must assert this input to the core for one cycle in response to the assertion of power state change interrupt, when it is ready to transition to the low-power state requested by the configuration write request. The client may permanently maintain this input high if it does not need to delay the return of the completions for the configuration write transactions causing power-state changes."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PwrStcgAck::B1)
    }
}
#[doc = "Power state change ack mode select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PwrStcgAckMode {
    #[doc = "0: select power state change ack source from bit pwr_stcg_ack"]
    B0 = 0,
    #[doc = "1: select power state change ack source from constant, it always keeps high."]
    B1 = 1,
}
impl From<PwrStcgAckMode> for bool {
    #[inline(always)]
    fn from(variant: PwrStcgAckMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWR_STCG_ACK_MODE` reader - Power state change ack mode select"]
pub type PwrStcgAckModeR = crate::BitReader<PwrStcgAckMode>;
impl PwrStcgAckModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PwrStcgAckMode {
        match self.bits {
            false => PwrStcgAckMode::B0,
            true => PwrStcgAckMode::B1,
        }
    }
    #[doc = "select power state change ack source from bit pwr_stcg_ack"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PwrStcgAckMode::B0
    }
    #[doc = "select power state change ack source from constant, it always keeps high."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PwrStcgAckMode::B1
    }
}
#[doc = "Field `PWR_STCG_ACK_MODE` writer - Power state change ack mode select"]
pub type PwrStcgAckModeW<'a, REG> = crate::BitWriter<'a, REG, PwrStcgAckMode>;
impl<'a, REG> PwrStcgAckModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "select power state change ack source from bit pwr_stcg_ack"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PwrStcgAckMode::B0)
    }
    #[doc = "select power state change ack source from constant, it always keeps high."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PwrStcgAckMode::B1)
    }
}
#[doc = "Write mask bits\n\nFor each served bit\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum WriteMask {
    #[doc = "0: write mask"]
    B0 = 0,
    #[doc = "1: write enable"]
    B1 = 1,
}
impl From<WriteMask> for u16 {
    #[inline(always)]
    fn from(variant: WriteMask) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WriteMask {
    type Ux = u16;
}
#[doc = "Field `WRITE_MASK` writer - Write mask bits\n\nFor each served bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, WriteMask>;
impl<'a, REG> WriteMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "write mask"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(WriteMask::B0)
    }
    #[doc = "write enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(WriteMask::B1)
    }
}
impl R {
    #[doc = "Bit 0 - Client request exit L2 power state"]
    #[inline(always)]
    pub fn clt_req_exit_l2(&self) -> CltReqExitL2R {
        CltReqExitL2R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Request transition to L23_Ready state"]
    #[inline(always)]
    pub fn req_trn_l23ready(&self) -> ReqTrnL23readyR {
        ReqTrnL23readyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Client request exit L1 power state"]
    #[inline(always)]
    pub fn clt_req_exit_l1(&self) -> CltReqExitL1R {
        CltReqExitL1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Hardware clear exit L2 request"]
    #[inline(always)]
    pub fn hwclr_exit_l2_req(&self) -> HwclrExitL2ReqR {
        HwclrExitL2ReqR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Hardware clear exit L1 request"]
    #[inline(always)]
    pub fn hwclr_exit_l1_req(&self) -> HwclrExitL1ReqR {
        HwclrExitL1ReqR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - Power state change ack mode select"]
    #[inline(always)]
    pub fn pwr_stcg_ack_mode(&self) -> PwrStcgAckModeR {
        PwrStcgAckModeR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Client request exit L2 power state"]
    #[inline(always)]
    #[must_use]
    pub fn clt_req_exit_l2(&mut self) -> CltReqExitL2W<PowerCtrlSpec> {
        CltReqExitL2W::new(self, 0)
    }
    #[doc = "Bit 1 - Request transition to L23_Ready state"]
    #[inline(always)]
    #[must_use]
    pub fn req_trn_l23ready(&mut self) -> ReqTrnL23readyW<PowerCtrlSpec> {
        ReqTrnL23readyW::new(self, 1)
    }
    #[doc = "Bit 2 - Client request exit L1 power state"]
    #[inline(always)]
    #[must_use]
    pub fn clt_req_exit_l1(&mut self) -> CltReqExitL1W<PowerCtrlSpec> {
        CltReqExitL1W::new(self, 2)
    }
    #[doc = "Bit 4 - Hardware clear exit L2 request"]
    #[inline(always)]
    #[must_use]
    pub fn hwclr_exit_l2_req(&mut self) -> HwclrExitL2ReqW<PowerCtrlSpec> {
        HwclrExitL2ReqW::new(self, 4)
    }
    #[doc = "Bit 6 - Hardware clear exit L1 request"]
    #[inline(always)]
    #[must_use]
    pub fn hwclr_exit_l1_req(&mut self) -> HwclrExitL1ReqW<PowerCtrlSpec> {
        HwclrExitL1ReqW::new(self, 6)
    }
    #[doc = "Bit 8 - Power state change ack"]
    #[inline(always)]
    #[must_use]
    pub fn pwr_stcg_ack(&mut self) -> PwrStcgAckW<PowerCtrlSpec> {
        PwrStcgAckW::new(self, 8)
    }
    #[doc = "Bit 9 - Power state change ack mode select"]
    #[inline(always)]
    #[must_use]
    pub fn pwr_stcg_ack_mode(&mut self) -> PwrStcgAckModeW<PowerCtrlSpec> {
        PwrStcgAckModeW::new(self, 9)
    }
    #[doc = "Bits 16:31 - Write mask bits\n\nFor each served bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<PowerCtrlSpec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "PCIe client power control configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`power_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`power_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PowerCtrlSpec;
impl crate::RegisterSpec for PowerCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`power_ctrl::R`](R) reader structure"]
impl crate::Readable for PowerCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`power_ctrl::W`](W) writer structure"]
impl crate::Writable for PowerCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets POWER_CTRL to value 0"]
impl crate::Resettable for PowerCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
