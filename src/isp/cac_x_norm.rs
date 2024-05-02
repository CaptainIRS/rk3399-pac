#[doc = "Register `CAC_X_NORM` reader"]
pub type R = crate::R<CacXNormSpec>;
#[doc = "Register `CAC_X_NORM` writer"]
pub type W = crate::W<CacXNormSpec>;
#[doc = "Field `x_nf` reader - Horizontal scaling or normalization factor x_nf (5 bit\n\nunsigned integer) range 0 .. 31 in equation x_d\\[7:0\\]
= (((h_count &lt;&lt; 4) >> x_ns) * x_nf) >> 5"]
pub type XNfR = crate::FieldReader;
#[doc = "Field `x_nf` writer - Horizontal scaling or normalization factor x_nf (5 bit\n\nunsigned integer) range 0 .. 31 in equation x_d\\[7:0\\]
= (((h_count &lt;&lt; 4) >> x_ns) * x_nf) >> 5"]
pub type XNfW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `x_ns` reader - Horizontal normalization shift parameter x_ns (4 bit\n\nunsigned integer) in equation x_d\\[7:0\\]
= (((h_count &lt;&lt; 4) >> x_ns) * x_nf) >> 5"]
pub type XNsR = crate::FieldReader;
#[doc = "Field `x_ns` writer - Horizontal normalization shift parameter x_ns (4 bit\n\nunsigned integer) in equation x_d\\[7:0\\]
= (((h_count &lt;&lt; 4) >> x_ns) * x_nf) >> 5"]
pub type XNsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:4 - Horizontal scaling or normalization factor x_nf (5 bit\n\nunsigned integer) range 0 .. 31 in equation x_d\\[7:0\\]
= (((h_count &lt;&lt; 4) >> x_ns) * x_nf) >> 5"]
    #[inline(always)]
    pub fn x_nf(&self) -> XNfR {
        XNfR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 16:19 - Horizontal normalization shift parameter x_ns (4 bit\n\nunsigned integer) in equation x_d\\[7:0\\]
= (((h_count &lt;&lt; 4) >> x_ns) * x_nf) >> 5"]
    #[inline(always)]
    pub fn x_ns(&self) -> XNsR {
        XNsR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Horizontal scaling or normalization factor x_nf (5 bit\n\nunsigned integer) range 0 .. 31 in equation x_d\\[7:0\\]
= (((h_count &lt;&lt; 4) >> x_ns) * x_nf) >> 5"]
    #[inline(always)]
    #[must_use]
    pub fn x_nf(&mut self) -> XNfW<CacXNormSpec> {
        XNfW::new(self, 0)
    }
    #[doc = "Bits 16:19 - Horizontal normalization shift parameter x_ns (4 bit\n\nunsigned integer) in equation x_d\\[7:0\\]
= (((h_count &lt;&lt; 4) >> x_ns) * x_nf) >> 5"]
    #[inline(always)]
    #[must_use]
    pub fn x_ns(&mut self) -> XNsW<CacXNormSpec> {
        XNsW::new(self, 16)
    }
}
#[doc = "Normalization parameters for calculation of image \n\n\n\ncoordinate x_d relative to optical center\n\nNote: These values need to be programmed according to the image resolution and the \n\ncenter offset of the chromatic aberration. \n\n\n\nThe parameters are necessary to avoid high gate count of the CAC hardware block. The \n\nreset value is valid for an image resolution of 2600 x 1950 and center offset 0. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cac_x_norm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cac_x_norm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CacXNormSpec;
impl crate::RegisterSpec for CacXNormSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cac_x_norm::R`](R) reader structure"]
impl crate::Readable for CacXNormSpec {}
#[doc = "`write(|w| ..)` method takes [`cac_x_norm::W`](W) writer structure"]
impl crate::Writable for CacXNormSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CAC_X_NORM to value 0x0008_0010"]
impl crate::Resettable for CacXNormSpec {
    const RESET_VALUE: u32 = 0x0008_0010;
}
