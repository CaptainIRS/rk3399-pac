#[doc = "Register `CAC_Y_NORM` reader"]
pub type R = crate::R<CacYNormSpec>;
#[doc = "Register `CAC_Y_NORM` writer"]
pub type W = crate::W<CacYNormSpec>;
#[doc = "Field `y_nf` reader - Vertical scaling or normalization factor y_nf (5 bit\n\nunsigned integer) range 0 .. 31 in equation y_d\\[7:0\\]
= (((v_count &lt;&lt; 4) >> y_ns) * y_nf) >> 5"]
pub type YNfR = crate::FieldReader;
#[doc = "Field `y_nf` writer - Vertical scaling or normalization factor y_nf (5 bit\n\nunsigned integer) range 0 .. 31 in equation y_d\\[7:0\\]
= (((v_count &lt;&lt; 4) >> y_ns) * y_nf) >> 5"]
pub type YNfW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `y_ns` reader - Vertical normalization shift parameter y_ns (4 bit\n\nunsigned integer) in equation y_d\\[7:0\\]
= (((v_count &lt;&lt; 4) >> y_ns) * y_nf) >> 5\n\n\n\n"]
pub type YNsR = crate::FieldReader;
#[doc = "Field `y_ns` writer - Vertical normalization shift parameter y_ns (4 bit\n\nunsigned integer) in equation y_d\\[7:0\\]
= (((v_count &lt;&lt; 4) >> y_ns) * y_nf) >> 5\n\n\n\n"]
pub type YNsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:4 - Vertical scaling or normalization factor y_nf (5 bit\n\nunsigned integer) range 0 .. 31 in equation y_d\\[7:0\\]
= (((v_count &lt;&lt; 4) >> y_ns) * y_nf) >> 5"]
    #[inline(always)]
    pub fn y_nf(&self) -> YNfR {
        YNfR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 16:19 - Vertical normalization shift parameter y_ns (4 bit\n\nunsigned integer) in equation y_d\\[7:0\\]
= (((v_count &lt;&lt; 4) >> y_ns) * y_nf) >> 5\n\n\n\n"]
    #[inline(always)]
    pub fn y_ns(&self) -> YNsR {
        YNsR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Vertical scaling or normalization factor y_nf (5 bit\n\nunsigned integer) range 0 .. 31 in equation y_d\\[7:0\\]
= (((v_count &lt;&lt; 4) >> y_ns) * y_nf) >> 5"]
    #[inline(always)]
    #[must_use]
    pub fn y_nf(&mut self) -> YNfW<CacYNormSpec> {
        YNfW::new(self, 0)
    }
    #[doc = "Bits 16:19 - Vertical normalization shift parameter y_ns (4 bit\n\nunsigned integer) in equation y_d\\[7:0\\]
= (((v_count &lt;&lt; 4) >> y_ns) * y_nf) >> 5\n\n\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn y_ns(&mut self) -> YNsW<CacYNormSpec> {
        YNsW::new(self, 16)
    }
}
#[doc = "Normalization parameters for calculation of image \n\n\n\ncoordinate y_d relative to optical center\n\nNote: These values need to be programmed according to the image resolution and the \n\ncenter offset of the chromatic aberration. \n\n\n\nThe parameters are necessary to avoid high gate count of the CAC hardware block. The \n\n\n\nreset value is valid for an image resolution of 2600 x 1950 and center offset 0. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cac_y_norm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cac_y_norm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CacYNormSpec;
impl crate::RegisterSpec for CacYNormSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cac_y_norm::R`](R) reader structure"]
impl crate::Readable for CacYNormSpec {}
#[doc = "`write(|w| ..)` method takes [`cac_y_norm::W`](W) writer structure"]
impl crate::Writable for CacYNormSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CAC_Y_NORM to value 0x0008_0010"]
impl crate::Resettable for CacYNormSpec {
    const RESET_VALUE: u32 = 0x0008_0010;
}
