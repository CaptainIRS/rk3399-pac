#[doc = "Register `DDR_DENALI_CTL_23` reader"]
pub type R = crate::R<DdrDenaliCtl23Spec>;
#[doc = "Register `DDR_DENALI_CTL_23` writer"]
pub type W = crate::W<DdrDenaliCtl23Spec>;
#[doc = "Field `TDLL_F2` reader - DRAM TDLL value for frequency copy 2 in cycles."]
pub type TdllF2R = crate::FieldReader<u16>;
#[doc = "Field `TDLL_F2` writer - DRAM TDLL value for frequency copy 2 in cycles."]
pub type TdllF2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CASLAT_LIN_F0` reader - Sets latency from read command send to data receive from/to controller for frequency copy 0. Bit (0) is half-cycle increment and the upper bits define memory CAS latency for the controller."]
pub type CaslatLinF0R = crate::FieldReader;
#[doc = "Field `CASLAT_LIN_F0` writer - Sets latency from read command send to data receive from/to controller for frequency copy 0. Bit (0) is half-cycle increment and the upper bits define memory CAS latency for the controller."]
pub type CaslatLinF0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `WRLAT_F0` reader - DRAM WRLAT value for frequency copy 0 in cycles."]
pub type WrlatF0R = crate::FieldReader;
#[doc = "Field `WRLAT_F0` writer - DRAM WRLAT value for frequency copy 0 in cycles."]
pub type WrlatF0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:15 - DRAM TDLL value for frequency copy 2 in cycles."]
    #[inline(always)]
    pub fn tdll_f2(&self) -> TdllF2R {
        TdllF2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:22 - Sets latency from read command send to data receive from/to controller for frequency copy 0. Bit (0) is half-cycle increment and the upper bits define memory CAS latency for the controller."]
    #[inline(always)]
    pub fn caslat_lin_f0(&self) -> CaslatLinF0R {
        CaslatLinF0R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:28 - DRAM WRLAT value for frequency copy 0 in cycles."]
    #[inline(always)]
    pub fn wrlat_f0(&self) -> WrlatF0R {
        WrlatF0R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - DRAM TDLL value for frequency copy 2 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tdll_f2(&mut self) -> TdllF2W<DdrDenaliCtl23Spec> {
        TdllF2W::new(self, 0)
    }
    #[doc = "Bits 16:22 - Sets latency from read command send to data receive from/to controller for frequency copy 0. Bit (0) is half-cycle increment and the upper bits define memory CAS latency for the controller."]
    #[inline(always)]
    #[must_use]
    pub fn caslat_lin_f0(&mut self) -> CaslatLinF0W<DdrDenaliCtl23Spec> {
        CaslatLinF0W::new(self, 16)
    }
    #[doc = "Bits 24:28 - DRAM WRLAT value for frequency copy 0 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn wrlat_f0(&mut self) -> WrlatF0W<DdrDenaliCtl23Spec> {
        WrlatF0W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_23::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_23::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl23Spec;
impl crate::RegisterSpec for DdrDenaliCtl23Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_23::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl23Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_23::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl23Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_23 to value 0"]
impl crate::Resettable for DdrDenaliCtl23Spec {
    const RESET_VALUE: u32 = 0;
}
