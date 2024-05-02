#[doc = "Register `GAMMA_DX_HI` reader"]
pub type R = crate::R<GammaDxHiSpec>;
#[doc = "Register `GAMMA_DX_HI` writer"]
pub type W = crate::W<GammaDxHiSpec>;
#[doc = "Field `GAMMA_DX_9` reader - gamma curve sample point definition x-axis (input)"]
pub type GammaDx9R = crate::FieldReader;
#[doc = "Field `GAMMA_DX_9` writer - gamma curve sample point definition x-axis (input)"]
pub type GammaDx9W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GAMMA_DX_10` reader - gamma curve sample point definition x-axis (input)"]
pub type GammaDx10R = crate::FieldReader;
#[doc = "Field `GAMMA_DX_10` writer - gamma curve sample point definition x-axis (input)"]
pub type GammaDx10W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GAMMA_DX_11` reader - gamma curve sample point definition x-axis (input)"]
pub type GammaDx11R = crate::FieldReader;
#[doc = "Field `GAMMA_DX_11` writer - gamma curve sample point definition x-axis (input)"]
pub type GammaDx11W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GAMMA_DX_12` reader - gamma curve sample point definition x-axis (input)"]
pub type GammaDx12R = crate::FieldReader;
#[doc = "Field `GAMMA_DX_12` writer - gamma curve sample point definition x-axis (input)"]
pub type GammaDx12W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GAMMA_DX_13` reader - gamma curve sample point definition x-axis (input)"]
pub type GammaDx13R = crate::FieldReader;
#[doc = "Field `GAMMA_DX_13` writer - gamma curve sample point definition x-axis (input)"]
pub type GammaDx13W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GAMMA_DX_14` reader - gamma curve sample point definition x-axis (input)"]
pub type GammaDx14R = crate::FieldReader;
#[doc = "Field `GAMMA_DX_14` writer - gamma curve sample point definition x-axis (input)"]
pub type GammaDx14W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GAMMA_DX_15` reader - gamma curve sample point definition x-axis (input)"]
pub type GammaDx15R = crate::FieldReader;
#[doc = "Field `GAMMA_DX_15` writer - gamma curve sample point definition x-axis (input)"]
pub type GammaDx15W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GAMMA_DX_16` reader - gamma curve sample point definition x-axis (input)"]
pub type GammaDx16R = crate::FieldReader;
#[doc = "Field `GAMMA_DX_16` writer - gamma curve sample point definition x-axis (input)"]
pub type GammaDx16W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - gamma curve sample point definition x-axis (input)"]
    #[inline(always)]
    pub fn gamma_dx_9(&self) -> GammaDx9R {
        GammaDx9R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - gamma curve sample point definition x-axis (input)"]
    #[inline(always)]
    pub fn gamma_dx_10(&self) -> GammaDx10R {
        GammaDx10R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - gamma curve sample point definition x-axis (input)"]
    #[inline(always)]
    pub fn gamma_dx_11(&self) -> GammaDx11R {
        GammaDx11R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - gamma curve sample point definition x-axis (input)"]
    #[inline(always)]
    pub fn gamma_dx_12(&self) -> GammaDx12R {
        GammaDx12R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - gamma curve sample point definition x-axis (input)"]
    #[inline(always)]
    pub fn gamma_dx_13(&self) -> GammaDx13R {
        GammaDx13R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - gamma curve sample point definition x-axis (input)"]
    #[inline(always)]
    pub fn gamma_dx_14(&self) -> GammaDx14R {
        GammaDx14R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - gamma curve sample point definition x-axis (input)"]
    #[inline(always)]
    pub fn gamma_dx_15(&self) -> GammaDx15R {
        GammaDx15R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - gamma curve sample point definition x-axis (input)"]
    #[inline(always)]
    pub fn gamma_dx_16(&self) -> GammaDx16R {
        GammaDx16R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - gamma curve sample point definition x-axis (input)"]
    #[inline(always)]
    #[must_use]
    pub fn gamma_dx_9(&mut self) -> GammaDx9W<GammaDxHiSpec> {
        GammaDx9W::new(self, 0)
    }
    #[doc = "Bits 4:6 - gamma curve sample point definition x-axis (input)"]
    #[inline(always)]
    #[must_use]
    pub fn gamma_dx_10(&mut self) -> GammaDx10W<GammaDxHiSpec> {
        GammaDx10W::new(self, 4)
    }
    #[doc = "Bits 8:10 - gamma curve sample point definition x-axis (input)"]
    #[inline(always)]
    #[must_use]
    pub fn gamma_dx_11(&mut self) -> GammaDx11W<GammaDxHiSpec> {
        GammaDx11W::new(self, 8)
    }
    #[doc = "Bits 12:14 - gamma curve sample point definition x-axis (input)"]
    #[inline(always)]
    #[must_use]
    pub fn gamma_dx_12(&mut self) -> GammaDx12W<GammaDxHiSpec> {
        GammaDx12W::new(self, 12)
    }
    #[doc = "Bits 16:18 - gamma curve sample point definition x-axis (input)"]
    #[inline(always)]
    #[must_use]
    pub fn gamma_dx_13(&mut self) -> GammaDx13W<GammaDxHiSpec> {
        GammaDx13W::new(self, 16)
    }
    #[doc = "Bits 20:22 - gamma curve sample point definition x-axis (input)"]
    #[inline(always)]
    #[must_use]
    pub fn gamma_dx_14(&mut self) -> GammaDx14W<GammaDxHiSpec> {
        GammaDx14W::new(self, 20)
    }
    #[doc = "Bits 24:26 - gamma curve sample point definition x-axis (input)"]
    #[inline(always)]
    #[must_use]
    pub fn gamma_dx_15(&mut self) -> GammaDx15W<GammaDxHiSpec> {
        GammaDx15W::new(self, 24)
    }
    #[doc = "Bits 28:30 - gamma curve sample point definition x-axis (input)"]
    #[inline(always)]
    #[must_use]
    pub fn gamma_dx_16(&mut self) -> GammaDx16W<GammaDxHiSpec> {
        GammaDx16W::new(self, 28)
    }
}
#[doc = "De-Gamma Curve definition higher x increments (sampling points)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gamma_dx_hi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gamma_dx_hi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GammaDxHiSpec;
impl crate::RegisterSpec for GammaDxHiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gamma_dx_hi::R`](R) reader structure"]
impl crate::Readable for GammaDxHiSpec {}
#[doc = "`write(|w| ..)` method takes [`gamma_dx_hi::W`](W) writer structure"]
impl crate::Writable for GammaDxHiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GAMMA_DX_HI to value 0x4444_4444"]
impl crate::Resettable for GammaDxHiSpec {
    const RESET_VALUE: u32 = 0x4444_4444;
}
