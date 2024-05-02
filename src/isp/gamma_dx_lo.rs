#[doc = "Register `GAMMA_DX_LO` reader"]
pub type R = crate::R<GammaDxLoSpec>;
#[doc = "Register `GAMMA_DX_LO` writer"]
pub type W = crate::W<GammaDxLoSpec>;
#[doc = "Field `GAMMA_DX_1` reader - gamma curve sample point definition x-axis (input)\n\n"]
pub type GammaDx1R = crate::FieldReader;
#[doc = "Field `GAMMA_DX_1` writer - gamma curve sample point definition x-axis (input)\n\n"]
pub type GammaDx1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GAMMA_DX_2` reader - gamma curve sample point definition x-axis (input)\n\n"]
pub type GammaDx2R = crate::FieldReader;
#[doc = "Field `GAMMA_DX_2` writer - gamma curve sample point definition x-axis (input)\n\n"]
pub type GammaDx2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GAMMA_DX_3` reader - gamma curve sample point definition x-axis (input)"]
pub type GammaDx3R = crate::FieldReader;
#[doc = "Field `GAMMA_DX_3` writer - gamma curve sample point definition x-axis (input)"]
pub type GammaDx3W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GAMMA_DX_4` reader - gamma curve sample point definition x-axis (input)"]
pub type GammaDx4R = crate::FieldReader;
#[doc = "Field `GAMMA_DX_4` writer - gamma curve sample point definition x-axis (input)"]
pub type GammaDx4W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GAMMA_DX_5` reader - gamma curve sample point definition x-axis (input)"]
pub type GammaDx5R = crate::FieldReader;
#[doc = "Field `GAMMA_DX_5` writer - gamma curve sample point definition x-axis (input)"]
pub type GammaDx5W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GAMMA_DX_6` reader - gamma curve sample point definition x-axis (input)"]
pub type GammaDx6R = crate::FieldReader;
#[doc = "Field `GAMMA_DX_6` writer - gamma curve sample point definition x-axis (input)"]
pub type GammaDx6W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GAMMA_DX_7` reader - gamma curve sample point definition x-axis (input)"]
pub type GammaDx7R = crate::FieldReader;
#[doc = "Field `GAMMA_DX_7` writer - gamma curve sample point definition x-axis (input)"]
pub type GammaDx7W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GAMMA_DX_8` reader - gamma curve sample point definition x-axis (input)"]
pub type GammaDx8R = crate::FieldReader;
#[doc = "Field `GAMMA_DX_8` writer - gamma curve sample point definition x-axis (input)"]
pub type GammaDx8W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - gamma curve sample point definition x-axis (input)\n\n"]
    #[inline(always)]
    pub fn gamma_dx_1(&self) -> GammaDx1R {
        GammaDx1R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - gamma curve sample point definition x-axis (input)\n\n"]
    #[inline(always)]
    pub fn gamma_dx_2(&self) -> GammaDx2R {
        GammaDx2R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - gamma curve sample point definition x-axis (input)"]
    #[inline(always)]
    pub fn gamma_dx_3(&self) -> GammaDx3R {
        GammaDx3R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - gamma curve sample point definition x-axis (input)"]
    #[inline(always)]
    pub fn gamma_dx_4(&self) -> GammaDx4R {
        GammaDx4R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - gamma curve sample point definition x-axis (input)"]
    #[inline(always)]
    pub fn gamma_dx_5(&self) -> GammaDx5R {
        GammaDx5R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - gamma curve sample point definition x-axis (input)"]
    #[inline(always)]
    pub fn gamma_dx_6(&self) -> GammaDx6R {
        GammaDx6R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - gamma curve sample point definition x-axis (input)"]
    #[inline(always)]
    pub fn gamma_dx_7(&self) -> GammaDx7R {
        GammaDx7R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - gamma curve sample point definition x-axis (input)"]
    #[inline(always)]
    pub fn gamma_dx_8(&self) -> GammaDx8R {
        GammaDx8R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - gamma curve sample point definition x-axis (input)\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn gamma_dx_1(&mut self) -> GammaDx1W<GammaDxLoSpec> {
        GammaDx1W::new(self, 0)
    }
    #[doc = "Bits 4:6 - gamma curve sample point definition x-axis (input)\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn gamma_dx_2(&mut self) -> GammaDx2W<GammaDxLoSpec> {
        GammaDx2W::new(self, 4)
    }
    #[doc = "Bits 8:10 - gamma curve sample point definition x-axis (input)"]
    #[inline(always)]
    #[must_use]
    pub fn gamma_dx_3(&mut self) -> GammaDx3W<GammaDxLoSpec> {
        GammaDx3W::new(self, 8)
    }
    #[doc = "Bits 12:14 - gamma curve sample point definition x-axis (input)"]
    #[inline(always)]
    #[must_use]
    pub fn gamma_dx_4(&mut self) -> GammaDx4W<GammaDxLoSpec> {
        GammaDx4W::new(self, 12)
    }
    #[doc = "Bits 16:18 - gamma curve sample point definition x-axis (input)"]
    #[inline(always)]
    #[must_use]
    pub fn gamma_dx_5(&mut self) -> GammaDx5W<GammaDxLoSpec> {
        GammaDx5W::new(self, 16)
    }
    #[doc = "Bits 20:22 - gamma curve sample point definition x-axis (input)"]
    #[inline(always)]
    #[must_use]
    pub fn gamma_dx_6(&mut self) -> GammaDx6W<GammaDxLoSpec> {
        GammaDx6W::new(self, 20)
    }
    #[doc = "Bits 24:26 - gamma curve sample point definition x-axis (input)"]
    #[inline(always)]
    #[must_use]
    pub fn gamma_dx_7(&mut self) -> GammaDx7W<GammaDxLoSpec> {
        GammaDx7W::new(self, 24)
    }
    #[doc = "Bits 28:30 - gamma curve sample point definition x-axis (input)"]
    #[inline(always)]
    #[must_use]
    pub fn gamma_dx_8(&mut self) -> GammaDx8W<GammaDxLoSpec> {
        GammaDx8W::new(self, 28)
    }
}
#[doc = "De-Gamma Curve definition lower x increments (sampling points)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gamma_dx_lo::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gamma_dx_lo::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GammaDxLoSpec;
impl crate::RegisterSpec for GammaDxLoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gamma_dx_lo::R`](R) reader structure"]
impl crate::Readable for GammaDxLoSpec {}
#[doc = "`write(|w| ..)` method takes [`gamma_dx_lo::W`](W) writer structure"]
impl crate::Writable for GammaDxLoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GAMMA_DX_LO to value 0x4444_4444"]
impl crate::Resettable for GammaDxLoSpec {
    const RESET_VALUE: u32 = 0x4444_4444;
}
