#[doc = "Register `GAMMA_OUT_MODE` reader"]
pub type R = crate::R<GammaOutModeSpec>;
#[doc = "Register `GAMMA_OUT_MODE` writer"]
pub type W = crate::W<GammaOutModeSpec>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EquSegm {
    #[doc = "0: logarithmic like segmentation of gamma curve (default after reset) segmentation from 0 to 4095: 64 64 64 64 128 128 128 128 256 256 256 512 512 512 512 512"]
    B0 = 0,
    #[doc = "1: equidistant segmentation (all 16 segments are 256 )"]
    B1 = 1,
}
impl From<EquSegm> for bool {
    #[inline(always)]
    fn from(variant: EquSegm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `equ_segm` reader - "]
pub type EquSegmR = crate::BitReader<EquSegm>;
impl EquSegmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EquSegm {
        match self.bits {
            false => EquSegm::B0,
            true => EquSegm::B1,
        }
    }
    #[doc = "logarithmic like segmentation of gamma curve (default after reset) segmentation from 0 to 4095: 64 64 64 64 128 128 128 128 256 256 256 512 512 512 512 512"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == EquSegm::B0
    }
    #[doc = "equidistant segmentation (all 16 segments are 256 )"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == EquSegm::B1
    }
}
#[doc = "Field `equ_segm` writer - "]
pub type EquSegmW<'a, REG> = crate::BitWriter<'a, REG, EquSegm>;
impl<'a, REG> EquSegmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "logarithmic like segmentation of gamma curve (default after reset) segmentation from 0 to 4095: 64 64 64 64 128 128 128 128 256 256 256 512 512 512 512 512"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(EquSegm::B0)
    }
    #[doc = "equidistant segmentation (all 16 segments are 256 )"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(EquSegm::B1)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn equ_segm(&self) -> EquSegmR {
        EquSegmR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn equ_segm(&mut self) -> EquSegmW<GammaOutModeSpec> {
        EquSegmW::new(self, 0)
    }
}
#[doc = "gamma segmentation mode register for output gamma\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gamma_out_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gamma_out_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GammaOutModeSpec;
impl crate::RegisterSpec for GammaOutModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gamma_out_mode::R`](R) reader structure"]
impl crate::Readable for GammaOutModeSpec {}
#[doc = "`write(|w| ..)` method takes [`gamma_out_mode::W`](W) writer structure"]
impl crate::Writable for GammaOutModeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GAMMA_OUT_MODE to value 0"]
impl crate::Resettable for GammaOutModeSpec {
    const RESET_VALUE: u32 = 0;
}
