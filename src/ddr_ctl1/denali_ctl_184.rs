#[doc = "Register `DENALI_CTL_184` reader"]
pub type R = crate::R<DenaliCtl184Spec>;
#[doc = "Register `DENALI_CTL_184` writer"]
pub type W = crate::W<DenaliCtl184Spec>;
#[doc = "Field `TZQLAT_F1` reader - Holds the DRAM ZQLAT value for frequency copy 1 in cycles."]
pub type TzqlatF1R = crate::FieldReader;
#[doc = "Field `TZQLAT_F1` writer - Holds the DRAM ZQLAT value for frequency copy 1 in cycles."]
pub type TzqlatF1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ZQINIT_F2` reader - Number of cycles needed for a ZQINIT command for frequency copy 2."]
pub type ZqinitF2R = crate::FieldReader<u16>;
#[doc = "Field `ZQINIT_F2` writer - Number of cycles needed for a ZQINIT command for frequency copy 2."]
pub type ZqinitF2W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:5 - Holds the DRAM ZQLAT value for frequency copy 1 in cycles."]
    #[inline(always)]
    pub fn tzqlat_f1(&self) -> TzqlatF1R {
        TzqlatF1R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:19 - Number of cycles needed for a ZQINIT command for frequency copy 2."]
    #[inline(always)]
    pub fn zqinit_f2(&self) -> ZqinitF2R {
        ZqinitF2R::new(((self.bits >> 8) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:5 - Holds the DRAM ZQLAT value for frequency copy 1 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tzqlat_f1(&mut self) -> TzqlatF1W<DenaliCtl184Spec> {
        TzqlatF1W::new(self, 0)
    }
    #[doc = "Bits 8:19 - Number of cycles needed for a ZQINIT command for frequency copy 2."]
    #[inline(always)]
    #[must_use]
    pub fn zqinit_f2(&mut self) -> ZqinitF2W<DenaliCtl184Spec> {
        ZqinitF2W::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_184::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_184::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl184Spec;
impl crate::RegisterSpec for DenaliCtl184Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_184::R`](R) reader structure"]
impl crate::Readable for DenaliCtl184Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_184::W`](W) writer structure"]
impl crate::Writable for DenaliCtl184Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_184 to value 0"]
impl crate::Resettable for DenaliCtl184Spec {
    const RESET_VALUE: u32 = 0;
}
