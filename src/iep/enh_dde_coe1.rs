#[doc = "Register `ENH_DDE_COE1` reader"]
pub type R = crate::R<EnhDdeCoe1Spec>;
#[doc = "Register `ENH_DDE_COE1` writer"]
pub type W = crate::W<EnhDdeCoe1Spec>;
#[doc = "Field `DDE_COE_0` reader - dde coefficient 1\n\n81x6bit cofficient for denoise and detail enhancement\n\ncoefficient number 0,4,8,12,......"]
pub type DdeCoe0R = crate::FieldReader;
#[doc = "Field `DDE_COE_0` writer - dde coefficient 1\n\n81x6bit cofficient for denoise and detail enhancement\n\ncoefficient number 0,4,8,12,......"]
pub type DdeCoe0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `DDE_COE_1` reader - dde coefficient 1\n\ncoefficient number 1,5,9,13,......"]
pub type DdeCoe1R = crate::FieldReader;
#[doc = "Field `DDE_COE_1` writer - dde coefficient 1\n\ncoefficient number 1,5,9,13,......"]
pub type DdeCoe1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `DDE_COE_2` reader - dde coefficient 3\n\ncoefficient number 2,6,10,14,......"]
pub type DdeCoe2R = crate::FieldReader;
#[doc = "Field `DDE_COE_2` writer - dde coefficient 3\n\ncoefficient number 2,6,10,14,......"]
pub type DdeCoe2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `DDE_COE_3` reader - dde coefficient 3\n\ncoefficient number 3,7,11,15,......"]
pub type DdeCoe3R = crate::FieldReader;
#[doc = "Field `DDE_COE_3` writer - dde coefficient 3\n\ncoefficient number 3,7,11,15,......"]
pub type DdeCoe3W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - dde coefficient 1\n\n81x6bit cofficient for denoise and detail enhancement\n\ncoefficient number 0,4,8,12,......"]
    #[inline(always)]
    pub fn dde_coe_0(&self) -> DdeCoe0R {
        DdeCoe0R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - dde coefficient 1\n\ncoefficient number 1,5,9,13,......"]
    #[inline(always)]
    pub fn dde_coe_1(&self) -> DdeCoe1R {
        DdeCoe1R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - dde coefficient 3\n\ncoefficient number 2,6,10,14,......"]
    #[inline(always)]
    pub fn dde_coe_2(&self) -> DdeCoe2R {
        DdeCoe2R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - dde coefficient 3\n\ncoefficient number 3,7,11,15,......"]
    #[inline(always)]
    pub fn dde_coe_3(&self) -> DdeCoe3R {
        DdeCoe3R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - dde coefficient 1\n\n81x6bit cofficient for denoise and detail enhancement\n\ncoefficient number 0,4,8,12,......"]
    #[inline(always)]
    #[must_use]
    pub fn dde_coe_0(&mut self) -> DdeCoe0W<EnhDdeCoe1Spec> {
        DdeCoe0W::new(self, 0)
    }
    #[doc = "Bits 8:13 - dde coefficient 1\n\ncoefficient number 1,5,9,13,......"]
    #[inline(always)]
    #[must_use]
    pub fn dde_coe_1(&mut self) -> DdeCoe1W<EnhDdeCoe1Spec> {
        DdeCoe1W::new(self, 8)
    }
    #[doc = "Bits 16:21 - dde coefficient 3\n\ncoefficient number 2,6,10,14,......"]
    #[inline(always)]
    #[must_use]
    pub fn dde_coe_2(&mut self) -> DdeCoe2W<EnhDdeCoe1Spec> {
        DdeCoe2W::new(self, 16)
    }
    #[doc = "Bits 24:29 - dde coefficient 3\n\ncoefficient number 3,7,11,15,......"]
    #[inline(always)]
    #[must_use]
    pub fn dde_coe_3(&mut self) -> DdeCoe3W<EnhDdeCoe1Spec> {
        DdeCoe3W::new(self, 24)
    }
}
#[doc = "denoise,detail and edge enhancement coefficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enh_dde_coe1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enh_dde_coe1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EnhDdeCoe1Spec;
impl crate::RegisterSpec for EnhDdeCoe1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`enh_dde_coe1::R`](R) reader structure"]
impl crate::Readable for EnhDdeCoe1Spec {}
#[doc = "`write(|w| ..)` method takes [`enh_dde_coe1::W`](W) writer structure"]
impl crate::Writable for EnhDdeCoe1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ENH_DDE_COE1 to value 0"]
impl crate::Resettable for EnhDdeCoe1Spec {
    const RESET_VALUE: u32 = 0;
}
