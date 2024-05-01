#[doc = "Register `DPI_COLOR_CODING` reader"]
pub type R = crate::R<DpiColorCodingSpec>;
#[doc = "Register `DPI_COLOR_CODING` writer"]
pub type W = crate::W<DpiColorCodingSpec>;
#[doc = "Field `DPI_COLOR_CODING` reader - dpi_color_coding\n\nThis field configures the DPI color coding as follows:\n\n■0000: 16-bit configuration 1\n\n■0001: 16-bit configuration 2\n\n■0010: 16-bit configuration 3\n\n■0011: 18-bit configuration 1\n\n■0100: 18-bit configuration 2\n\n■0101: 24-bit\n\n■0110: 20-bit YCbCr 4:2:2 loosely packed\n\n■0111: 24-bit YCbCr 4:2:2\n\n■1000: 16-bit YCbCr 4:2:2\n\n■1001: 30-bit\n\n■1010: 36-bit\n\n■1011-1111: 12-bit YCbCr 4:2:0"]
pub type DpiColorCodingR = crate::FieldReader;
#[doc = "Field `DPI_COLOR_CODING` writer - dpi_color_coding\n\nThis field configures the DPI color coding as follows:\n\n■0000: 16-bit configuration 1\n\n■0001: 16-bit configuration 2\n\n■0010: 16-bit configuration 3\n\n■0011: 18-bit configuration 1\n\n■0100: 18-bit configuration 2\n\n■0101: 24-bit\n\n■0110: 20-bit YCbCr 4:2:2 loosely packed\n\n■0111: 24-bit YCbCr 4:2:2\n\n■1000: 16-bit YCbCr 4:2:2\n\n■1001: 30-bit\n\n■1010: 36-bit\n\n■1011-1111: 12-bit YCbCr 4:2:0"]
pub type DpiColorCodingW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LOOSELY18_EN` reader - Field0001 Abstract\n\nWhen set to 1, this bit activates loosely packed variant to 18-bit\n\nconfigurations."]
pub type Loosely18EnR = crate::BitReader;
#[doc = "Field `LOOSELY18_EN` writer - Field0001 Abstract\n\nWhen set to 1, this bit activates loosely packed variant to 18-bit\n\nconfigurations."]
pub type Loosely18EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - dpi_color_coding\n\nThis field configures the DPI color coding as follows:\n\n■0000: 16-bit configuration 1\n\n■0001: 16-bit configuration 2\n\n■0010: 16-bit configuration 3\n\n■0011: 18-bit configuration 1\n\n■0100: 18-bit configuration 2\n\n■0101: 24-bit\n\n■0110: 20-bit YCbCr 4:2:2 loosely packed\n\n■0111: 24-bit YCbCr 4:2:2\n\n■1000: 16-bit YCbCr 4:2:2\n\n■1001: 30-bit\n\n■1010: 36-bit\n\n■1011-1111: 12-bit YCbCr 4:2:0"]
    #[inline(always)]
    pub fn dpi_color_coding(&self) -> DpiColorCodingR {
        DpiColorCodingR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Field0001 Abstract\n\nWhen set to 1, this bit activates loosely packed variant to 18-bit\n\nconfigurations."]
    #[inline(always)]
    pub fn loosely18_en(&self) -> Loosely18EnR {
        Loosely18EnR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - dpi_color_coding\n\nThis field configures the DPI color coding as follows:\n\n■0000: 16-bit configuration 1\n\n■0001: 16-bit configuration 2\n\n■0010: 16-bit configuration 3\n\n■0011: 18-bit configuration 1\n\n■0100: 18-bit configuration 2\n\n■0101: 24-bit\n\n■0110: 20-bit YCbCr 4:2:2 loosely packed\n\n■0111: 24-bit YCbCr 4:2:2\n\n■1000: 16-bit YCbCr 4:2:2\n\n■1001: 30-bit\n\n■1010: 36-bit\n\n■1011-1111: 12-bit YCbCr 4:2:0"]
    #[inline(always)]
    #[must_use]
    pub fn dpi_color_coding(&mut self) -> DpiColorCodingW<DpiColorCodingSpec> {
        DpiColorCodingW::new(self, 0)
    }
    #[doc = "Bit 8 - Field0001 Abstract\n\nWhen set to 1, this bit activates loosely packed variant to 18-bit\n\nconfigurations."]
    #[inline(always)]
    #[must_use]
    pub fn loosely18_en(&mut self) -> Loosely18EnW<DpiColorCodingSpec> {
        Loosely18EnW::new(self, 8)
    }
}
#[doc = "DPI Color Coding Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpi_color_coding::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpi_color_coding::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpiColorCodingSpec;
impl crate::RegisterSpec for DpiColorCodingSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpi_color_coding::R`](R) reader structure"]
impl crate::Readable for DpiColorCodingSpec {}
#[doc = "`write(|w| ..)` method takes [`dpi_color_coding::W`](W) writer structure"]
impl crate::Writable for DpiColorCodingSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DPI_COLOR_CODING to value 0"]
impl crate::Resettable for DpiColorCodingSpec {
    const RESET_VALUE: u32 = 0;
}
