#[doc = "Register `SPDIF_USRDRn` reader"]
pub type R = crate::R<SpdifUsrdrnSpec>;
#[doc = "Register `SPDIF_USRDRn` writer"]
pub type W = crate::W<SpdifUsrdrnSpec>;
#[doc = "Field `USR_SUB_0` reader - User Data Subframe 0\n\nUser Data Bit for Subframe 0"]
pub type UsrSub0R = crate::FieldReader<u16>;
#[doc = "Field `USR_SUB_0` writer - User Data Subframe 0\n\nUser Data Bit for Subframe 0"]
pub type UsrSub0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `USR_SUB_1` reader - User Data Subframe 1\n\nUser Data Bit for Subframe 1"]
pub type UsrSub1R = crate::FieldReader<u16>;
#[doc = "Field `USR_SUB_1` writer - User Data Subframe 1\n\nUser Data Bit for Subframe 1"]
pub type UsrSub1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - User Data Subframe 0\n\nUser Data Bit for Subframe 0"]
    #[inline(always)]
    pub fn usr_sub_0(&self) -> UsrSub0R {
        UsrSub0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - User Data Subframe 1\n\nUser Data Bit for Subframe 1"]
    #[inline(always)]
    pub fn usr_sub_1(&self) -> UsrSub1R {
        UsrSub1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - User Data Subframe 0\n\nUser Data Bit for Subframe 0"]
    #[inline(always)]
    #[must_use]
    pub fn usr_sub_0(&mut self) -> UsrSub0W<SpdifUsrdrnSpec> {
        UsrSub0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - User Data Subframe 1\n\nUser Data Bit for Subframe 1"]
    #[inline(always)]
    #[must_use]
    pub fn usr_sub_1(&mut self) -> UsrSub1W<SpdifUsrdrnSpec> {
        UsrSub1W::new(self, 16)
    }
}
#[doc = "User Data Register n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spdif_usrdrn::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spdif_usrdrn::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpdifUsrdrnSpec;
impl crate::RegisterSpec for SpdifUsrdrnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spdif_usrdrn::R`](R) reader structure"]
impl crate::Readable for SpdifUsrdrnSpec {}
#[doc = "`write(|w| ..)` method takes [`spdif_usrdrn::W`](W) writer structure"]
impl crate::Writable for SpdifUsrdrnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPDIF_USRDRn to value 0"]
impl crate::Resettable for SpdifUsrdrnSpec {
    const RESET_VALUE: u32 = 0;
}
