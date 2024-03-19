#[doc = "Register `DDR_PI_REG_41` reader"]
pub type R = crate::R<DdrPiReg41Spec>;
#[doc = "Register `DDR_PI_REG_41` writer"]
pub type W = crate::W<DdrPiReg41Spec>;
#[doc = "Field `PI_WDT_DISABLE` reader - Disables the watchdog caused by phymstr_req/phylvl_req_cs_n response error. Set 1 to disable."]
pub type PiWdtDisableR = crate::BitReader;
#[doc = "Field `PI_WDT_DISABLE` writer - Disables the watchdog caused by phymstr_req/phylvl_req_cs_n response error. Set 1 to disable."]
pub type PiWdtDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_SW_RST_N` reader - Indicates user request to reset the whole system except parameter DFFs. Set 0 to reset, set to 1 to release."]
pub type PiSwRstNR = crate::BitReader;
#[doc = "Field `PI_SW_RST_N` writer - Indicates user request to reset the whole system except parameter DFFs. Set 0 to reset, set to 1 to release."]
pub type PiSwRstNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_CS_MAP` reader - Defines the chip selects that are active."]
pub type PiCsMapR = crate::FieldReader;
#[doc = "Field `PI_CS_MAP` writer - Defines the chip selects that are active."]
pub type PiCsMapW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Disables the watchdog caused by phymstr_req/phylvl_req_cs_n response error. Set 1 to disable."]
    #[inline(always)]
    pub fn pi_wdt_disable(&self) -> PiWdtDisableR {
        PiWdtDisableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Indicates user request to reset the whole system except parameter DFFs. Set 0 to reset, set to 1 to release."]
    #[inline(always)]
    pub fn pi_sw_rst_n(&self) -> PiSwRstNR {
        PiSwRstNR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Defines the chip selects that are active."]
    #[inline(always)]
    pub fn pi_cs_map(&self) -> PiCsMapR {
        PiCsMapR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Disables the watchdog caused by phymstr_req/phylvl_req_cs_n response error. Set 1 to disable."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wdt_disable(&mut self) -> PiWdtDisableW<DdrPiReg41Spec> {
        PiWdtDisableW::new(self, 0)
    }
    #[doc = "Bit 8 - Indicates user request to reset the whole system except parameter DFFs. Set 0 to reset, set to 1 to release."]
    #[inline(always)]
    #[must_use]
    pub fn pi_sw_rst_n(&mut self) -> PiSwRstNW<DdrPiReg41Spec> {
        PiSwRstNW::new(self, 8)
    }
    #[doc = "Bits 24:25 - Defines the chip selects that are active."]
    #[inline(always)]
    #[must_use]
    pub fn pi_cs_map(&mut self) -> PiCsMapW<DdrPiReg41Spec> {
        PiCsMapW::new(self, 24)
    }
}
#[doc = "DDR PHY Independent Register 41\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_41::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_41::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg41Spec;
impl crate::RegisterSpec for DdrPiReg41Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_41::R`](R) reader structure"]
impl crate::Readable for DdrPiReg41Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_41::W`](W) writer structure"]
impl crate::Writable for DdrPiReg41Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_41 to value 0x0101"]
impl crate::Resettable for DdrPiReg41Spec {
    const RESET_VALUE: u32 = 0x0101;
}
