#[doc = "Register `EMMCCORE_BLKSIZ` reader"]
pub type R = crate::R<EmmccoreBlksizSpec>;
#[doc = "Register `EMMCCORE_BLKSIZ` writer"]
pub type W = crate::W<EmmccoreBlksizSpec>;
#[doc = "Field `TRANSFERBLOCKSIZE` reader - This register specifies the block size for block data transfers for CMD17, CMD18, CMD24, CMD25, and CMD53. It can be accessed only if no transaction is executing (i.e after a transaction has stopped). Read operations during transfer return an invalid value and write operations shall be ignored. 12'h0000: No Data Transfer 12'h0001: 1 Byte 12'h0002: 2 Bytes 12'h0003: 3 Bytes 12'h0004: 4 Bytes ........ 12'h01FF: 511 Bytes 12'h0200: 512 Bytes ........ 12'h0800: 2048 Bytes"]
pub type TransferblocksizeR = crate::FieldReader<u16>;
#[doc = "Field `TRANSFERBLOCKSIZE` writer - This register specifies the block size for block data transfers for CMD17, CMD18, CMD24, CMD25, and CMD53. It can be accessed only if no transaction is executing (i.e after a transaction has stopped). Read operations during transfer return an invalid value and write operations shall be ignored. 12'h0000: No Data Transfer 12'h0001: 1 Byte 12'h0002: 2 Bytes 12'h0003: 3 Bytes 12'h0004: 4 Bytes ........ 12'h01FF: 511 Bytes 12'h0200: 512 Bytes ........ 12'h0800: 2048 Bytes"]
pub type TransferblocksizeW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "To perform long DMA transfer, System Address register shall be updated at every system boundary during DMA transfer. These bits specify the size of contiguous buffer in the system memory. The DMA transfer shall wait at the every boundary specified by these fields and the HC generates the DMA Interrupt to request the HD to update the System Address register. These bits shall support when the DMA Support in the Capabilities register is set to 1 and this function is active when the DMA Enable in the Transfer Mode register is set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hostsdmabuffersize {
    #[doc = "0: 512KB(Detects A18 Carry out)"]
    H0 = 0,
    #[doc = "1: 512KB(Detects A18 Carry out)"]
    H1 = 1,
    #[doc = "2: 512KB(Detects A18 Carry out)"]
    H2 = 2,
    #[doc = "3: 512KB(Detects A18 Carry out)"]
    H3 = 3,
    #[doc = "4: 512KB(Detects A18 Carry out)"]
    H4 = 4,
    #[doc = "5: 512KB(Detects A18 Carry out)"]
    H5 = 5,
    #[doc = "6: 512KB(Detects A18 Carry out)"]
    H6 = 6,
    #[doc = "7: 512KB(Detects A18 Carry out)"]
    H7 = 7,
}
impl From<Hostsdmabuffersize> for u8 {
    #[inline(always)]
    fn from(variant: Hostsdmabuffersize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hostsdmabuffersize {
    type Ux = u8;
}
#[doc = "Field `HOSTSDMABUFFERSIZE` reader - To perform long DMA transfer, System Address register shall be updated at every system boundary during DMA transfer. These bits specify the size of contiguous buffer in the system memory. The DMA transfer shall wait at the every boundary specified by these fields and the HC generates the DMA Interrupt to request the HD to update the System Address register. These bits shall support when the DMA Support in the Capabilities register is set to 1 and this function is active when the DMA Enable in the Transfer Mode register is set to 1."]
pub type HostsdmabuffersizeR = crate::FieldReader<Hostsdmabuffersize>;
impl HostsdmabuffersizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hostsdmabuffersize {
        match self.bits {
            0 => Hostsdmabuffersize::H0,
            1 => Hostsdmabuffersize::H1,
            2 => Hostsdmabuffersize::H2,
            3 => Hostsdmabuffersize::H3,
            4 => Hostsdmabuffersize::H4,
            5 => Hostsdmabuffersize::H5,
            6 => Hostsdmabuffersize::H6,
            7 => Hostsdmabuffersize::H7,
            _ => unreachable!(),
        }
    }
    #[doc = "512KB(Detects A18 Carry out)"]
    #[inline(always)]
    pub fn is_h0(&self) -> bool {
        *self == Hostsdmabuffersize::H0
    }
    #[doc = "512KB(Detects A18 Carry out)"]
    #[inline(always)]
    pub fn is_h1(&self) -> bool {
        *self == Hostsdmabuffersize::H1
    }
    #[doc = "512KB(Detects A18 Carry out)"]
    #[inline(always)]
    pub fn is_h2(&self) -> bool {
        *self == Hostsdmabuffersize::H2
    }
    #[doc = "512KB(Detects A18 Carry out)"]
    #[inline(always)]
    pub fn is_h3(&self) -> bool {
        *self == Hostsdmabuffersize::H3
    }
    #[doc = "512KB(Detects A18 Carry out)"]
    #[inline(always)]
    pub fn is_h4(&self) -> bool {
        *self == Hostsdmabuffersize::H4
    }
    #[doc = "512KB(Detects A18 Carry out)"]
    #[inline(always)]
    pub fn is_h5(&self) -> bool {
        *self == Hostsdmabuffersize::H5
    }
    #[doc = "512KB(Detects A18 Carry out)"]
    #[inline(always)]
    pub fn is_h6(&self) -> bool {
        *self == Hostsdmabuffersize::H6
    }
    #[doc = "512KB(Detects A18 Carry out)"]
    #[inline(always)]
    pub fn is_h7(&self) -> bool {
        *self == Hostsdmabuffersize::H7
    }
}
#[doc = "Field `HOSTSDMABUFFERSIZE` writer - To perform long DMA transfer, System Address register shall be updated at every system boundary during DMA transfer. These bits specify the size of contiguous buffer in the system memory. The DMA transfer shall wait at the every boundary specified by these fields and the HC generates the DMA Interrupt to request the HD to update the System Address register. These bits shall support when the DMA Support in the Capabilities register is set to 1 and this function is active when the DMA Enable in the Transfer Mode register is set to 1."]
pub type HostsdmabuffersizeW<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, Hostsdmabuffersize>;
impl<'a, REG> HostsdmabuffersizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "512KB(Detects A18 Carry out)"]
    #[inline(always)]
    pub fn h0(self) -> &'a mut crate::W<REG> {
        self.variant(Hostsdmabuffersize::H0)
    }
    #[doc = "512KB(Detects A18 Carry out)"]
    #[inline(always)]
    pub fn h1(self) -> &'a mut crate::W<REG> {
        self.variant(Hostsdmabuffersize::H1)
    }
    #[doc = "512KB(Detects A18 Carry out)"]
    #[inline(always)]
    pub fn h2(self) -> &'a mut crate::W<REG> {
        self.variant(Hostsdmabuffersize::H2)
    }
    #[doc = "512KB(Detects A18 Carry out)"]
    #[inline(always)]
    pub fn h3(self) -> &'a mut crate::W<REG> {
        self.variant(Hostsdmabuffersize::H3)
    }
    #[doc = "512KB(Detects A18 Carry out)"]
    #[inline(always)]
    pub fn h4(self) -> &'a mut crate::W<REG> {
        self.variant(Hostsdmabuffersize::H4)
    }
    #[doc = "512KB(Detects A18 Carry out)"]
    #[inline(always)]
    pub fn h5(self) -> &'a mut crate::W<REG> {
        self.variant(Hostsdmabuffersize::H5)
    }
    #[doc = "512KB(Detects A18 Carry out)"]
    #[inline(always)]
    pub fn h6(self) -> &'a mut crate::W<REG> {
        self.variant(Hostsdmabuffersize::H6)
    }
    #[doc = "512KB(Detects A18 Carry out)"]
    #[inline(always)]
    pub fn h7(self) -> &'a mut crate::W<REG> {
        self.variant(Hostsdmabuffersize::H7)
    }
}
impl R {
    #[doc = "Bits 0:11 - This register specifies the block size for block data transfers for CMD17, CMD18, CMD24, CMD25, and CMD53. It can be accessed only if no transaction is executing (i.e after a transaction has stopped). Read operations during transfer return an invalid value and write operations shall be ignored. 12'h0000: No Data Transfer 12'h0001: 1 Byte 12'h0002: 2 Bytes 12'h0003: 3 Bytes 12'h0004: 4 Bytes ........ 12'h01FF: 511 Bytes 12'h0200: 512 Bytes ........ 12'h0800: 2048 Bytes"]
    #[inline(always)]
    pub fn transferblocksize(&self) -> TransferblocksizeR {
        TransferblocksizeR::new(self.bits & 0x0fff)
    }
    #[doc = "Bits 12:14 - To perform long DMA transfer, System Address register shall be updated at every system boundary during DMA transfer. These bits specify the size of contiguous buffer in the system memory. The DMA transfer shall wait at the every boundary specified by these fields and the HC generates the DMA Interrupt to request the HD to update the System Address register. These bits shall support when the DMA Support in the Capabilities register is set to 1 and this function is active when the DMA Enable in the Transfer Mode register is set to 1."]
    #[inline(always)]
    pub fn hostsdmabuffersize(&self) -> HostsdmabuffersizeR {
        HostsdmabuffersizeR::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - This register specifies the block size for block data transfers for CMD17, CMD18, CMD24, CMD25, and CMD53. It can be accessed only if no transaction is executing (i.e after a transaction has stopped). Read operations during transfer return an invalid value and write operations shall be ignored. 12'h0000: No Data Transfer 12'h0001: 1 Byte 12'h0002: 2 Bytes 12'h0003: 3 Bytes 12'h0004: 4 Bytes ........ 12'h01FF: 511 Bytes 12'h0200: 512 Bytes ........ 12'h0800: 2048 Bytes"]
    #[inline(always)]
    #[must_use]
    pub fn transferblocksize(&mut self) -> TransferblocksizeW<EmmccoreBlksizSpec> {
        TransferblocksizeW::new(self, 0)
    }
    #[doc = "Bits 12:14 - To perform long DMA transfer, System Address register shall be updated at every system boundary during DMA transfer. These bits specify the size of contiguous buffer in the system memory. The DMA transfer shall wait at the every boundary specified by these fields and the HC generates the DMA Interrupt to request the HD to update the System Address register. These bits shall support when the DMA Support in the Capabilities register is set to 1 and this function is active when the DMA Enable in the Transfer Mode register is set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn hostsdmabuffersize(&mut self) -> HostsdmabuffersizeW<EmmccoreBlksizSpec> {
        HostsdmabuffersizeW::new(self, 12)
    }
}
#[doc = "Block size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_blksiz::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_blksiz::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmmccoreBlksizSpec;
impl crate::RegisterSpec for EmmccoreBlksizSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`emmccore_blksiz::R`](R) reader structure"]
impl crate::Readable for EmmccoreBlksizSpec {}
#[doc = "`write(|w| ..)` method takes [`emmccore_blksiz::W`](W) writer structure"]
impl crate::Writable for EmmccoreBlksizSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets EMMCCORE_BLKSIZ to value 0"]
impl crate::Resettable for EmmccoreBlksizSpec {
    const RESET_VALUE: u16 = 0;
}
