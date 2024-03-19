#[doc = "Register `DDR_DENALI_CTL_26` reader"]
pub type R = crate::R<DdrDenaliCtl26Spec>;
#[doc = "Register `DDR_DENALI_CTL_26` writer"]
pub type W = crate::W<DdrDenaliCtl26Spec>;
#[doc = "Field `TCCD` reader - DRAM CAS-to-CAS value in cycles."]
pub type TccdR = crate::FieldReader;
#[doc = "Field `TCCD` writer - DRAM CAS-to-CAS value in cycles."]
pub type TccdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TCCDMW` reader - DRAM CAS-to-CAS masked write value in cycles."]
pub type TccdmwR = crate::FieldReader;
#[doc = "Field `TCCDMW` writer - DRAM CAS-to-CAS masked write value in cycles."]
pub type TccdmwW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `TRRD_F0` reader - DRAM TRRD value for frequency copy 0 in cycles."]
pub type TrrdF0R = crate::FieldReader;
#[doc = "Field `TRRD_F0` writer - DRAM TRRD value for frequency copy 0 in cycles."]
pub type TrrdF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TRC_F0` reader - DRAM TRC value for frequency copy 0 in cycles."]
pub type TrcF0R = crate::FieldReader;
#[doc = "Field `TRC_F0` writer - DRAM TRC value for frequency copy 0 in cycles."]
pub type TrcF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:4 - DRAM CAS-to-CAS value in cycles."]
    #[inline(always)]
    pub fn tccd(&self) -> TccdR {
        TccdR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:13 - DRAM CAS-to-CAS masked write value in cycles."]
    #[inline(always)]
    pub fn tccdmw(&self) -> TccdmwR {
        TccdmwR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:23 - DRAM TRRD value for frequency copy 0 in cycles."]
    #[inline(always)]
    pub fn trrd_f0(&self) -> TrrdF0R {
        TrrdF0R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - DRAM TRC value for frequency copy 0 in cycles."]
    #[inline(always)]
    pub fn trc_f0(&self) -> TrcF0R {
        TrcF0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - DRAM CAS-to-CAS value in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tccd(&mut self) -> TccdW<DdrDenaliCtl26Spec> {
        TccdW::new(self, 0)
    }
    #[doc = "Bits 8:13 - DRAM CAS-to-CAS masked write value in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tccdmw(&mut self) -> TccdmwW<DdrDenaliCtl26Spec> {
        TccdmwW::new(self, 8)
    }
    #[doc = "Bits 16:23 - DRAM TRRD value for frequency copy 0 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn trrd_f0(&mut self) -> TrrdF0W<DdrDenaliCtl26Spec> {
        TrrdF0W::new(self, 16)
    }
    #[doc = "Bits 24:31 - DRAM TRC value for frequency copy 0 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn trc_f0(&mut self) -> TrcF0W<DdrDenaliCtl26Spec> {
        TrcF0W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_26::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_26::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl26Spec;
impl crate::RegisterSpec for DdrDenaliCtl26Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_26::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl26Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_26::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl26Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_26 to value 0"]
impl crate::Resettable for DdrDenaliCtl26Spec {
    const RESET_VALUE: u32 = 0;
}
