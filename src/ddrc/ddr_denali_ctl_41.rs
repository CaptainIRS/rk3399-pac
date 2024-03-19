#[doc = "Register `DDR_DENALI_CTL_41` reader"]
pub type R = crate::R<DdrDenaliCtl41Spec>;
#[doc = "Register `DDR_DENALI_CTL_41` writer"]
pub type W = crate::W<DdrDenaliCtl41Spec>;
#[doc = "Field `TCACKEL` reader - DRAM TCACKEL value in cycles."]
pub type TcackelR = crate::FieldReader;
#[doc = "Field `TCACKEL` writer - DRAM TCACKEL value in cycles."]
pub type TcackelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TCAENT` reader - DRAM TCAENT value in cycles."]
pub type TcaentR = crate::FieldReader<u16>;
#[doc = "Field `TCAENT` writer - DRAM TCAENT value in cycles."]
pub type TcaentW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `TCAMRD` reader - DRAM TCAMRD value in cycles."]
pub type TcamrdR = crate::FieldReader;
#[doc = "Field `TCAMRD` writer - DRAM TCAMRD value in cycles."]
pub type TcamrdW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:4 - DRAM TCACKEL value in cycles."]
    #[inline(always)]
    pub fn tcackel(&self) -> TcackelR {
        TcackelR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:17 - DRAM TCAENT value in cycles."]
    #[inline(always)]
    pub fn tcaent(&self) -> TcaentR {
        TcaentR::new(((self.bits >> 8) & 0x03ff) as u16)
    }
    #[doc = "Bits 24:29 - DRAM TCAMRD value in cycles."]
    #[inline(always)]
    pub fn tcamrd(&self) -> TcamrdR {
        TcamrdR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - DRAM TCACKEL value in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tcackel(&mut self) -> TcackelW<DdrDenaliCtl41Spec> {
        TcackelW::new(self, 0)
    }
    #[doc = "Bits 8:17 - DRAM TCAENT value in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tcaent(&mut self) -> TcaentW<DdrDenaliCtl41Spec> {
        TcaentW::new(self, 8)
    }
    #[doc = "Bits 24:29 - DRAM TCAMRD value in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tcamrd(&mut self) -> TcamrdW<DdrDenaliCtl41Spec> {
        TcamrdW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_41::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_41::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl41Spec;
impl crate::RegisterSpec for DdrDenaliCtl41Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_41::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl41Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_41::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl41Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_41 to value 0"]
impl crate::Resettable for DdrDenaliCtl41Spec {
    const RESET_VALUE: u32 = 0;
}
