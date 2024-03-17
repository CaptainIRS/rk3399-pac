#[doc = "Register `DENALI_CTL_149` reader"]
pub type R = crate::R<DenaliCtl149Spec>;
#[doc = "Register `DENALI_CTL_149` writer"]
pub type W = crate::W<DenaliCtl149Spec>;
#[doc = "Field `MR1_DATA_F1_1` reader - Data to program into memory mode register 1 for chip select 1 for frequency copy 1."]
pub type Mr1DataF1_1R = crate::FieldReader<u16>;
#[doc = "Field `MR1_DATA_F1_1` writer - Data to program into memory mode register 1 for chip select 1 for frequency copy 1."]
pub type Mr1DataF1_1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MR2_DATA_F1_1` reader - Data to program into memory mode register 2 for chip select 1 for frequency copy 1."]
pub type Mr2DataF1_1R = crate::FieldReader<u16>;
#[doc = "Field `MR2_DATA_F1_1` writer - Data to program into memory mode register 2 for chip select 1 for frequency copy 1."]
pub type Mr2DataF1_1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Data to program into memory mode register 1 for chip select 1 for frequency copy 1."]
    #[inline(always)]
    pub fn mr1_data_f1_1(&self) -> Mr1DataF1_1R {
        Mr1DataF1_1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Data to program into memory mode register 2 for chip select 1 for frequency copy 1."]
    #[inline(always)]
    pub fn mr2_data_f1_1(&self) -> Mr2DataF1_1R {
        Mr2DataF1_1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data to program into memory mode register 1 for chip select 1 for frequency copy 1."]
    #[inline(always)]
    #[must_use]
    pub fn mr1_data_f1_1(&mut self) -> Mr1DataF1_1W<DenaliCtl149Spec> {
        Mr1DataF1_1W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Data to program into memory mode register 2 for chip select 1 for frequency copy 1."]
    #[inline(always)]
    #[must_use]
    pub fn mr2_data_f1_1(&mut self) -> Mr2DataF1_1W<DenaliCtl149Spec> {
        Mr2DataF1_1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_149::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_149::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl149Spec;
impl crate::RegisterSpec for DenaliCtl149Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_149::R`](R) reader structure"]
impl crate::Readable for DenaliCtl149Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_149::W`](W) writer structure"]
impl crate::Writable for DenaliCtl149Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_149 to value 0"]
impl crate::Resettable for DenaliCtl149Spec {
    const RESET_VALUE: u32 = 0;
}
