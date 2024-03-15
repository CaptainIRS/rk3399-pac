#[doc = "Register `DP_DEBUG_CTL` reader"]
pub type R = crate::R<DpDebugCtlSpec>;
#[doc = "Register `DP_DEBUG_CTL` writer"]
pub type W = crate::W<DpDebugCtlSpec>;
#[doc = "Invert SERDES output polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PnInv {
    #[doc = "1: Normal operation. This bit's type is R/W."]
    B1 = 1,
    #[doc = "0: Normal operation. This bit's type is R/W."]
    B0 = 0,
}
impl From<PnInv> for bool {
    #[inline(always)]
    fn from(variant: PnInv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PN_INV` reader - Invert SERDES output polarity"]
pub type PnInvR = crate::BitReader<PnInv>;
impl PnInvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PnInv {
        match self.bits {
            true => PnInv::B1,
            false => PnInv::B0,
        }
    }
    #[doc = "Normal operation. This bit's type is R/W."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PnInv::B1
    }
    #[doc = "Normal operation. This bit's type is R/W."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PnInv::B0
    }
}
#[doc = "Field `PN_INV` writer - Invert SERDES output polarity"]
pub type PnInvW<'a, REG> = crate::BitWriter<'a, REG, PnInv>;
impl<'a, REG> PnInvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation. This bit's type is R/W."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PnInv::B1)
    }
    #[doc = "Normal operation. This bit's type is R/W."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PnInv::B0)
    }
}
#[doc = "Enable hardware state machine to polling the HPD status or link status. The interval of each polling is controlled by POLLING_PERIOD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PollingEn {
    #[doc = "1: Disable polling function This bit's type is R/W."]
    B1 = 1,
    #[doc = "0: Disable polling function This bit's type is R/W."]
    B0 = 0,
}
impl From<PollingEn> for bool {
    #[inline(always)]
    fn from(variant: PollingEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POLLING_EN` reader - Enable hardware state machine to polling the HPD status or link status. The interval of each polling is controlled by POLLING_PERIOD"]
pub type PollingEnR = crate::BitReader<PollingEn>;
impl PollingEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PollingEn {
        match self.bits {
            true => PollingEn::B1,
            false => PollingEn::B0,
        }
    }
    #[doc = "Disable polling function This bit's type is R/W."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PollingEn::B1
    }
    #[doc = "Disable polling function This bit's type is R/W."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PollingEn::B0
    }
}
#[doc = "Field `POLLING_EN` writer - Enable hardware state machine to polling the HPD status or link status. The interval of each polling is controlled by POLLING_PERIOD"]
pub type PollingEnW<'a, REG> = crate::BitWriter<'a, REG, PollingEn>;
impl<'a, REG> PollingEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable polling function This bit's type is R/W."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PollingEn::B1)
    }
    #[doc = "Disable polling function This bit's type is R/W."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PollingEn::B0)
    }
}
#[doc = "PLL lock register control enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PllLockCtrl {
    #[doc = "1: PLL lock signal is controlled by PLL. This bit's type is R/W"]
    B1 = 1,
    #[doc = "0: PLL lock signal is controlled by PLL. This bit's type is R/W"]
    B0 = 0,
}
impl From<PllLockCtrl> for bool {
    #[inline(always)]
    fn from(variant: PllLockCtrl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLL_LOCK_CTRL` reader - PLL lock register control enable"]
pub type PllLockCtrlR = crate::BitReader<PllLockCtrl>;
impl PllLockCtrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PllLockCtrl {
        match self.bits {
            true => PllLockCtrl::B1,
            false => PllLockCtrl::B0,
        }
    }
    #[doc = "PLL lock signal is controlled by PLL. This bit's type is R/W"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PllLockCtrl::B1
    }
    #[doc = "PLL lock signal is controlled by PLL. This bit's type is R/W"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PllLockCtrl::B0
    }
}
#[doc = "Field `PLL_LOCK_CTRL` writer - PLL lock register control enable"]
pub type PllLockCtrlW<'a, REG> = crate::BitWriter1C<'a, REG, PllLockCtrl>;
impl<'a, REG> PllLockCtrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PLL lock signal is controlled by PLL. This bit's type is R/W"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PllLockCtrl::B1)
    }
    #[doc = "PLL lock signal is controlled by PLL. This bit's type is R/W"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PllLockCtrl::B0)
    }
}
#[doc = "Force PLL lock, this bit is active when PLL_LOCK_CTRL is 1:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPllLock {
    #[doc = "1: Force PLL non-lock. This bit's type is R/W"]
    B1 = 1,
    #[doc = "0: Force PLL non-lock. This bit's type is R/W"]
    B0 = 0,
}
impl From<FPllLock> for bool {
    #[inline(always)]
    fn from(variant: FPllLock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `F_PLL_LOCK` reader - Force PLL lock, this bit is active when PLL_LOCK_CTRL is 1:"]
pub type FPllLockR = crate::BitReader<FPllLock>;
impl FPllLockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FPllLock {
        match self.bits {
            true => FPllLock::B1,
            false => FPllLock::B0,
        }
    }
    #[doc = "Force PLL non-lock. This bit's type is R/W"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == FPllLock::B1
    }
    #[doc = "Force PLL non-lock. This bit's type is R/W"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == FPllLock::B0
    }
}
#[doc = "Field `F_PLL_LOCK` writer - Force PLL lock, this bit is active when PLL_LOCK_CTRL is 1:"]
pub type FPllLockW<'a, REG> = crate::BitWriter1C<'a, REG, FPllLock>;
impl<'a, REG> FPllLockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Force PLL non-lock. This bit's type is R/W"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(FPllLock::B1)
    }
    #[doc = "Force PLL non-lock. This bit's type is R/W"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(FPllLock::B0)
    }
}
#[doc = "PLL lock status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PllLock {
    #[doc = "1: PLL unlock. This bit's type is RO."]
    B1 = 1,
    #[doc = "0: PLL unlock. This bit's type is RO."]
    B0 = 0,
}
impl From<PllLock> for bool {
    #[inline(always)]
    fn from(variant: PllLock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLL_LOCK` reader - PLL lock status"]
pub type PllLockR = crate::BitReader<PllLock>;
impl PllLockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PllLock {
        match self.bits {
            true => PllLock::B1,
            false => PllLock::B0,
        }
    }
    #[doc = "PLL unlock. This bit's type is RO."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PllLock::B1
    }
    #[doc = "PLL unlock. This bit's type is RO."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PllLock::B0
    }
}
#[doc = "Bypass link status polling. If this bit, MYDP_HPD_POLLIN_EN and POLLING_EN are all enabled, hardware only polling MYDP HPD status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BypassStatusPolling {
    #[doc = "1: Disabled. This bit's type is RW."]
    B1 = 1,
    #[doc = "0: Disabled. This bit's type is RW."]
    B0 = 0,
}
impl From<BypassStatusPolling> for bool {
    #[inline(always)]
    fn from(variant: BypassStatusPolling) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BYPASS_STATUS_POLLING` reader - Bypass link status polling. If this bit, MYDP_HPD_POLLIN_EN and POLLING_EN are all enabled, hardware only polling MYDP HPD status."]
pub type BypassStatusPollingR = crate::BitReader<BypassStatusPolling>;
impl BypassStatusPollingR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BypassStatusPolling {
        match self.bits {
            true => BypassStatusPolling::B1,
            false => BypassStatusPolling::B0,
        }
    }
    #[doc = "Disabled. This bit's type is RW."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == BypassStatusPolling::B1
    }
    #[doc = "Disabled. This bit's type is RW."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == BypassStatusPolling::B0
    }
}
#[doc = "Field `BYPASS_STATUS_POLLING` writer - Bypass link status polling. If this bit, MYDP_HPD_POLLIN_EN and POLLING_EN are all enabled, hardware only polling MYDP HPD status."]
pub type BypassStatusPollingW<'a, REG> = crate::BitWriter<'a, REG, BypassStatusPolling>;
impl<'a, REG> BypassStatusPollingW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. This bit's type is RW."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(BypassStatusPolling::B1)
    }
    #[doc = "Disabled. This bit's type is RW."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(BypassStatusPolling::B0)
    }
}
#[doc = "Enable the MYDP HPD status polling. If this bit and POLLING_EN are enabled and BYPASS_STATUS_POLLING is 0, hardware polling both of link status and MYDP HPD status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MydpHpdPollinEn {
    #[doc = "1: Disabled. This bit's type is RW."]
    B1 = 1,
    #[doc = "0: Disabled. This bit's type is RW."]
    B0 = 0,
}
impl From<MydpHpdPollinEn> for bool {
    #[inline(always)]
    fn from(variant: MydpHpdPollinEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MYDP_HPD_POLLIN_EN` reader - Enable the MYDP HPD status polling. If this bit and POLLING_EN are enabled and BYPASS_STATUS_POLLING is 0, hardware polling both of link status and MYDP HPD status."]
pub type MydpHpdPollinEnR = crate::BitReader<MydpHpdPollinEn>;
impl MydpHpdPollinEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MydpHpdPollinEn {
        match self.bits {
            true => MydpHpdPollinEn::B1,
            false => MydpHpdPollinEn::B0,
        }
    }
    #[doc = "Disabled. This bit's type is RW."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == MydpHpdPollinEn::B1
    }
    #[doc = "Disabled. This bit's type is RW."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == MydpHpdPollinEn::B0
    }
}
#[doc = "Field `MYDP_HPD_POLLIN_EN` writer - Enable the MYDP HPD status polling. If this bit and POLLING_EN are enabled and BYPASS_STATUS_POLLING is 0, hardware polling both of link status and MYDP HPD status."]
pub type MydpHpdPollinEnW<'a, REG> = crate::BitWriter<'a, REG, MydpHpdPollinEn>;
impl<'a, REG> MydpHpdPollinEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. This bit's type is RW."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(MydpHpdPollinEn::B1)
    }
    #[doc = "Disabled. This bit's type is RW."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(MydpHpdPollinEn::B0)
    }
}
impl R {
    #[doc = "Bit 0 - Invert SERDES output polarity"]
    #[inline(always)]
    pub fn pn_inv(&self) -> PnInvR {
        PnInvR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable hardware state machine to polling the HPD status or link status. The interval of each polling is controlled by POLLING_PERIOD"]
    #[inline(always)]
    pub fn polling_en(&self) -> PollingEnR {
        PollingEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PLL lock register control enable"]
    #[inline(always)]
    pub fn pll_lock_ctrl(&self) -> PllLockCtrlR {
        PllLockCtrlR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Force PLL lock, this bit is active when PLL_LOCK_CTRL is 1:"]
    #[inline(always)]
    pub fn f_pll_lock(&self) -> FPllLockR {
        FPllLockR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PLL lock status"]
    #[inline(always)]
    pub fn pll_lock(&self) -> PllLockR {
        PllLockR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bypass link status polling. If this bit, MYDP_HPD_POLLIN_EN and POLLING_EN are all enabled, hardware only polling MYDP HPD status."]
    #[inline(always)]
    pub fn bypass_status_polling(&self) -> BypassStatusPollingR {
        BypassStatusPollingR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable the MYDP HPD status polling. If this bit and POLLING_EN are enabled and BYPASS_STATUS_POLLING is 0, hardware polling both of link status and MYDP HPD status."]
    #[inline(always)]
    pub fn mydp_hpd_pollin_en(&self) -> MydpHpdPollinEnR {
        MydpHpdPollinEnR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Invert SERDES output polarity"]
    #[inline(always)]
    #[must_use]
    pub fn pn_inv(&mut self) -> PnInvW<DpDebugCtlSpec> {
        PnInvW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable hardware state machine to polling the HPD status or link status. The interval of each polling is controlled by POLLING_PERIOD"]
    #[inline(always)]
    #[must_use]
    pub fn polling_en(&mut self) -> PollingEnW<DpDebugCtlSpec> {
        PollingEnW::new(self, 1)
    }
    #[doc = "Bit 2 - PLL lock register control enable"]
    #[inline(always)]
    #[must_use]
    pub fn pll_lock_ctrl(&mut self) -> PllLockCtrlW<DpDebugCtlSpec> {
        PllLockCtrlW::new(self, 2)
    }
    #[doc = "Bit 3 - Force PLL lock, this bit is active when PLL_LOCK_CTRL is 1:"]
    #[inline(always)]
    #[must_use]
    pub fn f_pll_lock(&mut self) -> FPllLockW<DpDebugCtlSpec> {
        FPllLockW::new(self, 3)
    }
    #[doc = "Bit 5 - Bypass link status polling. If this bit, MYDP_HPD_POLLIN_EN and POLLING_EN are all enabled, hardware only polling MYDP HPD status."]
    #[inline(always)]
    #[must_use]
    pub fn bypass_status_polling(&mut self) -> BypassStatusPollingW<DpDebugCtlSpec> {
        BypassStatusPollingW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable the MYDP HPD status polling. If this bit and POLLING_EN are enabled and BYPASS_STATUS_POLLING is 0, hardware polling both of link status and MYDP HPD status."]
    #[inline(always)]
    #[must_use]
    pub fn mydp_hpd_pollin_en(&mut self) -> MydpHpdPollinEnW<DpDebugCtlSpec> {
        MydpHpdPollinEnW::new(self, 6)
    }
}
#[doc = "DP Debug Control Register #1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dp_debug_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dp_debug_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpDebugCtlSpec;
impl crate::RegisterSpec for DpDebugCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dp_debug_ctl::R`](R) reader structure"]
impl crate::Readable for DpDebugCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`dp_debug_ctl::W`](W) writer structure"]
impl crate::Writable for DpDebugCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0c;
}
#[doc = "`reset()` method sets DP_DEBUG_CTL to value 0"]
impl crate::Resettable for DpDebugCtlSpec {
    const RESET_VALUE: u32 = 0;
}
