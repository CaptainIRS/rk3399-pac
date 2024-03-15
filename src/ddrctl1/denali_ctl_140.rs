#[doc = "Register `DENALI_CTL_140` reader"]
pub type R = crate::R<DenaliCtl140Spec>;
#[doc = "Register `DENALI_CTL_140` writer"]
pub type W = crate::W<DenaliCtl140Spec>;
#[doc = "Field `MR11_DATA_F1_0` reader - Data to program into memory mode register 11 for chip select 0 for frequency copy 1."]
pub type Mr11DataF1_0R = crate::FieldReader;
#[doc = "Field `MR11_DATA_F1_0` writer - Data to program into memory mode register 11 for chip select 0 for frequency copy 1."]
pub type Mr11DataF1_0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MR11_DATA_F2_0` reader - Data to program into memory mode register 11 for chip select 0 for frequency copy 2."]
pub type Mr11DataF2_0R = crate::FieldReader;
#[doc = "Field `MR11_DATA_F2_0` writer - Data to program into memory mode register 11 for chip select 0 for frequency copy 2."]
pub type Mr11DataF2_0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MR12_DATA_F0_0` reader - Data to program into memory mode register 12 for chip select 0."]
pub type Mr12DataF0_0R = crate::FieldReader<u16>;
#[doc = "Field `MR12_DATA_F0_0` writer - Data to program into memory mode register 12 for chip select 0."]
pub type Mr12DataF0_0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:7 - Data to program into memory mode register 11 for chip select 0 for frequency copy 1."]
    #[inline(always)]
    pub fn mr11_data_f1_0(&self) -> Mr11DataF1_0R {
        Mr11DataF1_0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Data to program into memory mode register 11 for chip select 0 for frequency copy 2."]
    #[inline(always)]
    pub fn mr11_data_f2_0(&self) -> Mr11DataF2_0R {
        Mr11DataF2_0R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Data to program into memory mode register 12 for chip select 0."]
    #[inline(always)]
    pub fn mr12_data_f0_0(&self) -> Mr12DataF0_0R {
        Mr12DataF0_0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data to program into memory mode register 11 for chip select 0 for frequency copy 1."]
    #[inline(always)]
    #[must_use]
    pub fn mr11_data_f1_0(&mut self) -> Mr11DataF1_0W<DenaliCtl140Spec> {
        Mr11DataF1_0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Data to program into memory mode register 11 for chip select 0 for frequency copy 2."]
    #[inline(always)]
    #[must_use]
    pub fn mr11_data_f2_0(&mut self) -> Mr11DataF2_0W<DenaliCtl140Spec> {
        Mr11DataF2_0W::new(self, 8)
    }
    #[doc = "Bits 16:31 - Data to program into memory mode register 12 for chip select 0."]
    #[inline(always)]
    #[must_use]
    pub fn mr12_data_f0_0(&mut self) -> Mr12DataF0_0W<DenaliCtl140Spec> {
        Mr12DataF0_0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_140::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_140::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl140Spec;
impl crate::RegisterSpec for DenaliCtl140Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_140::R`](R) reader structure"]
impl crate::Readable for DenaliCtl140Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_140::W`](W) writer structure"]
impl crate::Writable for DenaliCtl140Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_140 to value 0"]
impl crate::Resettable for DenaliCtl140Spec {
    const RESET_VALUE: u32 = 0;
}
