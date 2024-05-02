#[doc = "Register `MRSZ_SCALE_VC` reader"]
pub type R = crate::R<MrszScaleVcSpec>;
#[doc = "Register `MRSZ_SCALE_VC` writer"]
pub type W = crate::W<MrszScaleVcSpec>;
#[doc = "Field `scale_vc` reader - This register is set to the vertical chrominance\n\ndownscale factor or to the reciprocal of the vertical\n\nchrominance upscale factor"]
pub type ScaleVcR = crate::FieldReader<u16>;
#[doc = "Field `scale_vc` writer - This register is set to the vertical chrominance\n\ndownscale factor or to the reciprocal of the vertical\n\nchrominance upscale factor"]
pub type ScaleVcW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - This register is set to the vertical chrominance\n\ndownscale factor or to the reciprocal of the vertical\n\nchrominance upscale factor"]
    #[inline(always)]
    pub fn scale_vc(&self) -> ScaleVcR {
        ScaleVcR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This register is set to the vertical chrominance\n\ndownscale factor or to the reciprocal of the vertical\n\nchrominance upscale factor"]
    #[inline(always)]
    #[must_use]
    pub fn scale_vc(&mut self) -> ScaleVcW<MrszScaleVcSpec> {
        ScaleVcW::new(self, 0)
    }
}
#[doc = "vertical chrominance scale factor register\n\nNote: The size of the output picture is calculated as follows: (size_out - 1) / (size_in - 1)) \n\n\n\n= scale, \n\nwhere size_in/out is the width or heigth of the in/output picture. The values of the \n\n\n\nMRSZ_SCALE registers then have to be int(scale x 2^14)+1 \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mrsz_scale_vc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mrsz_scale_vc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MrszScaleVcSpec;
impl crate::RegisterSpec for MrszScaleVcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mrsz_scale_vc::R`](R) reader structure"]
impl crate::Readable for MrszScaleVcSpec {}
#[doc = "`write(|w| ..)` method takes [`mrsz_scale_vc::W`](W) writer structure"]
impl crate::Writable for MrszScaleVcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MRSZ_SCALE_VC to value 0"]
impl crate::Resettable for MrszScaleVcSpec {
    const RESET_VALUE: u32 = 0;
}
