#[doc = "Register `DENALI_CTL_151` reader"]
pub type R = crate::R<DenaliCtl151Spec>;
#[doc = "Register `DENALI_CTL_151` writer"]
pub type W = crate::W<DenaliCtl151Spec>;
#[doc = "Field `MR2_DATA_F2_1` reader - Data to program into memory mode register 2 for chip select 1 for frequency copy 2."]
pub type Mr2DataF2_1R = crate::FieldReader<u16>;
#[doc = "Field `MR2_DATA_F2_1` writer - Data to program into memory mode register 2 for chip select 1 for frequency copy 2."]
pub type Mr2DataF2_1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MRSINGLE_DATA_1` reader - Data to program into memory mode register single write to chip select 1."]
pub type MrsingleData1R = crate::FieldReader<u16>;
#[doc = "Field `MRSINGLE_DATA_1` writer - Data to program into memory mode register single write to chip select 1."]
pub type MrsingleData1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Data to program into memory mode register 2 for chip select 1 for frequency copy 2."]
    #[inline(always)]
    pub fn mr2_data_f2_1(&self) -> Mr2DataF2_1R {
        Mr2DataF2_1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Data to program into memory mode register single write to chip select 1."]
    #[inline(always)]
    pub fn mrsingle_data_1(&self) -> MrsingleData1R {
        MrsingleData1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data to program into memory mode register 2 for chip select 1 for frequency copy 2."]
    #[inline(always)]
    #[must_use]
    pub fn mr2_data_f2_1(&mut self) -> Mr2DataF2_1W<DenaliCtl151Spec> {
        Mr2DataF2_1W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Data to program into memory mode register single write to chip select 1."]
    #[inline(always)]
    #[must_use]
    pub fn mrsingle_data_1(&mut self) -> MrsingleData1W<DenaliCtl151Spec> {
        MrsingleData1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_151::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_151::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl151Spec;
impl crate::RegisterSpec for DenaliCtl151Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_151::R`](R) reader structure"]
impl crate::Readable for DenaliCtl151Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_151::W`](W) writer structure"]
impl crate::Writable for DenaliCtl151Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_151 to value 0"]
impl crate::Resettable for DenaliCtl151Spec {
    const RESET_VALUE: u32 = 0;
}
