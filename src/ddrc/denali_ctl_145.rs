#[doc = "Register `DENALI_CTL_145` reader"]
pub type R = crate::R<DenaliCtl145Spec>;
#[doc = "Register `DENALI_CTL_145` writer"]
pub type W = crate::W<DenaliCtl145Spec>;
#[doc = "Field `MR17_DATA_0` reader - Data to program into memory mode register 17 for chip select 0."]
pub type Mr17Data0R = crate::FieldReader;
#[doc = "Field `MR17_DATA_0` writer - Data to program into memory mode register 17 for chip select 0."]
pub type Mr17Data0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MR20_DATA_0` reader - Data read from MR20 for chip select 0."]
pub type Mr20Data0R = crate::FieldReader;
#[doc = "Field `MR22_DATA_F0_0` reader - Data to program into memory mode register 22 for chip select 0."]
pub type Mr22DataF0_0R = crate::FieldReader<u16>;
#[doc = "Field `MR22_DATA_F0_0` writer - Data to program into memory mode register 22 for chip select 0."]
pub type Mr22DataF0_0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:7 - Data to program into memory mode register 17 for chip select 0."]
    #[inline(always)]
    pub fn mr17_data_0(&self) -> Mr17Data0R {
        Mr17Data0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Data read from MR20 for chip select 0."]
    #[inline(always)]
    pub fn mr20_data_0(&self) -> Mr20Data0R {
        Mr20Data0R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Data to program into memory mode register 22 for chip select 0."]
    #[inline(always)]
    pub fn mr22_data_f0_0(&self) -> Mr22DataF0_0R {
        Mr22DataF0_0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data to program into memory mode register 17 for chip select 0."]
    #[inline(always)]
    #[must_use]
    pub fn mr17_data_0(&mut self) -> Mr17Data0W<DenaliCtl145Spec> {
        Mr17Data0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Data to program into memory mode register 22 for chip select 0."]
    #[inline(always)]
    #[must_use]
    pub fn mr22_data_f0_0(&mut self) -> Mr22DataF0_0W<DenaliCtl145Spec> {
        Mr22DataF0_0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_145::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_145::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl145Spec;
impl crate::RegisterSpec for DenaliCtl145Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_145::R`](R) reader structure"]
impl crate::Readable for DenaliCtl145Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_145::W`](W) writer structure"]
impl crate::Writable for DenaliCtl145Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_145 to value 0"]
impl crate::Resettable for DenaliCtl145Spec {
    const RESET_VALUE: u32 = 0;
}
