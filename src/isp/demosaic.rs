#[doc = "Register `DEMOSAIC` reader"]
pub type R = crate::R<DemosaicSpec>;
#[doc = "Register `DEMOSAIC` writer"]
pub type W = crate::W<DemosaicSpec>;
#[doc = "Field `DEMOSAIC_TH` reader - Threshold for Bayer demosaicing texture detection.\n\nThis value shifted left 4bit is compared with the difference\n\nof the vertical and horizontal 12Bit wide texture indicators,\n\nto decide if the vertical or horizontal texture flag must be\n\nset.\n\n0xFF: no texture detection 0x00: maximum edge\n\nsensitivity\n\n"]
pub type DemosaicThR = crate::FieldReader;
#[doc = "Field `DEMOSAIC_TH` writer - Threshold for Bayer demosaicing texture detection.\n\nThis value shifted left 4bit is compared with the difference\n\nof the vertical and horizontal 12Bit wide texture indicators,\n\nto decide if the vertical or horizontal texture flag must be\n\nset.\n\n0xFF: no texture detection 0x00: maximum edge\n\nsensitivity\n\n"]
pub type DemosaicThW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DemosaicBypass {
    #[doc = "0: normal operation for RGB Bayer Pattern input"]
    B0 = 0,
    #[doc = "1: demosaicing bypass for Black&amp;White input data"]
    B1 = 1,
}
impl From<DemosaicBypass> for bool {
    #[inline(always)]
    fn from(variant: DemosaicBypass) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEMOSAIC_BYPASS` reader - "]
pub type DemosaicBypassR = crate::BitReader<DemosaicBypass>;
impl DemosaicBypassR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DemosaicBypass {
        match self.bits {
            false => DemosaicBypass::B0,
            true => DemosaicBypass::B1,
        }
    }
    #[doc = "normal operation for RGB Bayer Pattern input"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DemosaicBypass::B0
    }
    #[doc = "demosaicing bypass for Black&amp;White input data"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DemosaicBypass::B1
    }
}
#[doc = "Field `DEMOSAIC_BYPASS` writer - "]
pub type DemosaicBypassW<'a, REG> = crate::BitWriter<'a, REG, DemosaicBypass>;
impl<'a, REG> DemosaicBypassW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal operation for RGB Bayer Pattern input"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DemosaicBypass::B0)
    }
    #[doc = "demosaicing bypass for Black&amp;White input data"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DemosaicBypass::B1)
    }
}
impl R {
    #[doc = "Bits 0:7 - Threshold for Bayer demosaicing texture detection.\n\nThis value shifted left 4bit is compared with the difference\n\nof the vertical and horizontal 12Bit wide texture indicators,\n\nto decide if the vertical or horizontal texture flag must be\n\nset.\n\n0xFF: no texture detection 0x00: maximum edge\n\nsensitivity\n\n"]
    #[inline(always)]
    pub fn demosaic_th(&self) -> DemosaicThR {
        DemosaicThR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn demosaic_bypass(&self) -> DemosaicBypassR {
        DemosaicBypassR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Threshold for Bayer demosaicing texture detection.\n\nThis value shifted left 4bit is compared with the difference\n\nof the vertical and horizontal 12Bit wide texture indicators,\n\nto decide if the vertical or horizontal texture flag must be\n\nset.\n\n0xFF: no texture detection 0x00: maximum edge\n\nsensitivity\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn demosaic_th(&mut self) -> DemosaicThW<DemosaicSpec> {
        DemosaicThW::new(self, 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn demosaic_bypass(&mut self) -> DemosaicBypassW<DemosaicSpec> {
        DemosaicBypassW::new(self, 10)
    }
}
#[doc = "Demosaic parameters\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`demosaic::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`demosaic::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DemosaicSpec;
impl crate::RegisterSpec for DemosaicSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`demosaic::R`](R) reader structure"]
impl crate::Readable for DemosaicSpec {}
#[doc = "`write(|w| ..)` method takes [`demosaic::W`](W) writer structure"]
impl crate::Writable for DemosaicSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEMOSAIC to value 0x04"]
impl crate::Resettable for DemosaicSpec {
    const RESET_VALUE: u32 = 0x04;
}
