#[doc = "Register `CRU_GLB_RST_CON` reader"]
pub type R = crate::R<CruGlbRstConSpec>;
#[doc = "Register `CRU_GLB_RST_CON` writer"]
pub type W = crate::W<CruGlbRstConSpec>;
#[doc = "TSADC trigger global soft reset select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TsadcGlbSrstCtrl {
    #[doc = "0: tsadc trigger first global reset"]
    B0 = 0,
    #[doc = "1: tsadc trigger first global reset"]
    B1 = 1,
}
impl From<TsadcGlbSrstCtrl> for bool {
    #[inline(always)]
    fn from(variant: TsadcGlbSrstCtrl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSADC_GLB_SRST_CTRL` reader - TSADC trigger global soft reset select"]
pub type TsadcGlbSrstCtrlR = crate::BitReader<TsadcGlbSrstCtrl>;
impl TsadcGlbSrstCtrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TsadcGlbSrstCtrl {
        match self.bits {
            false => TsadcGlbSrstCtrl::B0,
            true => TsadcGlbSrstCtrl::B1,
        }
    }
    #[doc = "tsadc trigger first global reset"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == TsadcGlbSrstCtrl::B0
    }
    #[doc = "tsadc trigger first global reset"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == TsadcGlbSrstCtrl::B1
    }
}
#[doc = "Field `TSADC_GLB_SRST_CTRL` writer - TSADC trigger global soft reset select"]
pub type TsadcGlbSrstCtrlW<'a, REG> = crate::BitWriter<'a, REG, TsadcGlbSrstCtrl>;
impl<'a, REG> TsadcGlbSrstCtrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "tsadc trigger first global reset"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(TsadcGlbSrstCtrl::B0)
    }
    #[doc = "tsadc trigger first global reset"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(TsadcGlbSrstCtrl::B1)
    }
}
#[doc = "watch_dog trigger global soft reset select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WdtGlbSrstCtrl {
    #[doc = "0: watch_dog trigger first global reset"]
    B0 = 0,
    #[doc = "1: watch_dog trigger first global reset"]
    B1 = 1,
}
impl From<WdtGlbSrstCtrl> for bool {
    #[inline(always)]
    fn from(variant: WdtGlbSrstCtrl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDT_GLB_SRST_CTRL` reader - watch_dog trigger global soft reset select"]
pub type WdtGlbSrstCtrlR = crate::BitReader<WdtGlbSrstCtrl>;
impl WdtGlbSrstCtrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WdtGlbSrstCtrl {
        match self.bits {
            false => WdtGlbSrstCtrl::B0,
            true => WdtGlbSrstCtrl::B1,
        }
    }
    #[doc = "watch_dog trigger first global reset"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == WdtGlbSrstCtrl::B0
    }
    #[doc = "watch_dog trigger first global reset"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == WdtGlbSrstCtrl::B1
    }
}
#[doc = "Field `WDT_GLB_SRST_CTRL` writer - watch_dog trigger global soft reset select"]
pub type WdtGlbSrstCtrlW<'a, REG> = crate::BitWriter<'a, REG, WdtGlbSrstCtrl>;
impl<'a, REG> WdtGlbSrstCtrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "watch_dog trigger first global reset"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(WdtGlbSrstCtrl::B0)
    }
    #[doc = "watch_dog trigger first global reset"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(WdtGlbSrstCtrl::B1)
    }
}
#[doc = "pmu reset by global soft reset select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PmuGlbSrstCtrl {
    #[doc = "0: pmu not reset by any global soft reset"]
    B00 = 0,
    #[doc = "1: pmu not reset by any global soft reset"]
    B01 = 1,
    #[doc = "2: pmu not reset by any global soft reset"]
    B10 = 2,
}
impl From<PmuGlbSrstCtrl> for u8 {
    #[inline(always)]
    fn from(variant: PmuGlbSrstCtrl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PmuGlbSrstCtrl {
    type Ux = u8;
}
#[doc = "Field `PMU_GLB_SRST_CTRL` reader - pmu reset by global soft reset select"]
pub type PmuGlbSrstCtrlR = crate::FieldReader<PmuGlbSrstCtrl>;
impl PmuGlbSrstCtrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PmuGlbSrstCtrl> {
        match self.bits {
            0 => Some(PmuGlbSrstCtrl::B00),
            1 => Some(PmuGlbSrstCtrl::B01),
            2 => Some(PmuGlbSrstCtrl::B10),
            _ => None,
        }
    }
    #[doc = "pmu not reset by any global soft reset"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == PmuGlbSrstCtrl::B00
    }
    #[doc = "pmu not reset by any global soft reset"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == PmuGlbSrstCtrl::B01
    }
    #[doc = "pmu not reset by any global soft reset"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == PmuGlbSrstCtrl::B10
    }
}
#[doc = "Field `PMU_GLB_SRST_CTRL` writer - pmu reset by global soft reset select"]
pub type PmuGlbSrstCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 2, PmuGlbSrstCtrl>;
impl<'a, REG> PmuGlbSrstCtrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "pmu not reset by any global soft reset"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(PmuGlbSrstCtrl::B00)
    }
    #[doc = "pmu not reset by any global soft reset"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(PmuGlbSrstCtrl::B01)
    }
    #[doc = "pmu not reset by any global soft reset"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(PmuGlbSrstCtrl::B10)
    }
}
#[doc = "if pmu reset by wdt resetn src select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PmuGlbrstWdtCtrl {
    #[doc = "0: pmu does not reset by wdt rstn"]
    B0 = 0,
    #[doc = "1: pmu does not reset by wdt rstn"]
    B1 = 1,
}
impl From<PmuGlbrstWdtCtrl> for bool {
    #[inline(always)]
    fn from(variant: PmuGlbrstWdtCtrl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PMU_GLBRST_WDT_CTRL` reader - if pmu reset by wdt resetn src select"]
pub type PmuGlbrstWdtCtrlR = crate::BitReader<PmuGlbrstWdtCtrl>;
impl PmuGlbrstWdtCtrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PmuGlbrstWdtCtrl {
        match self.bits {
            false => PmuGlbrstWdtCtrl::B0,
            true => PmuGlbrstWdtCtrl::B1,
        }
    }
    #[doc = "pmu does not reset by wdt rstn"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PmuGlbrstWdtCtrl::B0
    }
    #[doc = "pmu does not reset by wdt rstn"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PmuGlbrstWdtCtrl::B1
    }
}
#[doc = "Field `PMU_GLBRST_WDT_CTRL` writer - if pmu reset by wdt resetn src select"]
pub type PmuGlbrstWdtCtrlW<'a, REG> = crate::BitWriter<'a, REG, PmuGlbrstWdtCtrl>;
impl<'a, REG> PmuGlbrstWdtCtrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "pmu does not reset by wdt rstn"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PmuGlbrstWdtCtrl::B0)
    }
    #[doc = "pmu does not reset by wdt rstn"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PmuGlbrstWdtCtrl::B1)
    }
}
impl R {
    #[doc = "Bit 0 - TSADC trigger global soft reset select"]
    #[inline(always)]
    pub fn tsadc_glb_srst_ctrl(&self) -> TsadcGlbSrstCtrlR {
        TsadcGlbSrstCtrlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - watch_dog trigger global soft reset select"]
    #[inline(always)]
    pub fn wdt_glb_srst_ctrl(&self) -> WdtGlbSrstCtrlR {
        WdtGlbSrstCtrlR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - pmu reset by global soft reset select"]
    #[inline(always)]
    pub fn pmu_glb_srst_ctrl(&self) -> PmuGlbSrstCtrlR {
        PmuGlbSrstCtrlR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - if pmu reset by wdt resetn src select"]
    #[inline(always)]
    pub fn pmu_glbrst_wdt_ctrl(&self) -> PmuGlbrstWdtCtrlR {
        PmuGlbrstWdtCtrlR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TSADC trigger global soft reset select"]
    #[inline(always)]
    #[must_use]
    pub fn tsadc_glb_srst_ctrl(&mut self) -> TsadcGlbSrstCtrlW<CruGlbRstConSpec> {
        TsadcGlbSrstCtrlW::new(self, 0)
    }
    #[doc = "Bit 1 - watch_dog trigger global soft reset select"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_glb_srst_ctrl(&mut self) -> WdtGlbSrstCtrlW<CruGlbRstConSpec> {
        WdtGlbSrstCtrlW::new(self, 1)
    }
    #[doc = "Bits 2:3 - pmu reset by global soft reset select"]
    #[inline(always)]
    #[must_use]
    pub fn pmu_glb_srst_ctrl(&mut self) -> PmuGlbSrstCtrlW<CruGlbRstConSpec> {
        PmuGlbSrstCtrlW::new(self, 2)
    }
    #[doc = "Bit 4 - if pmu reset by wdt resetn src select"]
    #[inline(always)]
    #[must_use]
    pub fn pmu_glbrst_wdt_ctrl(&mut self) -> PmuGlbrstWdtCtrlW<CruGlbRstConSpec> {
        PmuGlbrstWdtCtrlW::new(self, 4)
    }
}
#[doc = "Global reset trigger select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_glb_rst_con::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_glb_rst_con::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruGlbRstConSpec;
impl crate::RegisterSpec for CruGlbRstConSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_glb_rst_con::R`](R) reader structure"]
impl crate::Readable for CruGlbRstConSpec {}
#[doc = "`write(|w| ..)` method takes [`cru_glb_rst_con::W`](W) writer structure"]
impl crate::Writable for CruGlbRstConSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_GLB_RST_CON to value 0"]
impl crate::Resettable for CruGlbRstConSpec {
    const RESET_VALUE: u32 = 0;
}
