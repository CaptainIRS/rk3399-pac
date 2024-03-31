#[doc = "Register `BLKSIZ` reader"]
pub type R = crate::R<BlksizSpec>;
#[doc = "Register `BLKSIZ` writer"]
pub type W = crate::W<BlksizSpec>;
#[doc = "Field `TRANSFERBLOCKSIZE` reader - This register specifies the block size for block data transfers for\n\nCMD17, CMD18, CMD24, CMD25, and CMD53. It can be accessed\n\nonly if no transaction is executing (i.e after a transaction has\n\nstopped). Read operations during transfer return an invalid value\n\nand write operations shall be ignored.\n\n12'h0000: No Data Transfer\n\n12'h0001: 1 Byte\n\n12'h0002: 2 Bytes\n\n12'h0003: 3 Bytes\n\n12'h0004: 4 Bytes\n\n........\n\n12'h01FF: 511 Bytes\n\n12'h0200: 512 Bytes\n\n........\n\n12'h0800: 2048 Bytes"]
pub type TransferblocksizeR = crate::FieldReader<u16>;
#[doc = "Field `TRANSFERBLOCKSIZE` writer - This register specifies the block size for block data transfers for\n\nCMD17, CMD18, CMD24, CMD25, and CMD53. It can be accessed\n\nonly if no transaction is executing (i.e after a transaction has\n\nstopped). Read operations during transfer return an invalid value\n\nand write operations shall be ignored.\n\n12'h0000: No Data Transfer\n\n12'h0001: 1 Byte\n\n12'h0002: 2 Bytes\n\n12'h0003: 3 Bytes\n\n12'h0004: 4 Bytes\n\n........\n\n12'h01FF: 511 Bytes\n\n12'h0200: 512 Bytes\n\n........\n\n12'h0800: 2048 Bytes"]
pub type TransferblocksizeW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "To perform long DMA transfer, System Address register shall be\n\nupdated at every system boundary during DMA transfer. These\n\nbits specify the size of contiguous buffer in the system memory.\n\nThe DMA transfer shall wait at the every boundary specified by\n\nthese fields and the HC generates the DMA Interrupt to request\n\nthe HD to update the System Address register.\n\nThese bits shall support when the DMA Support in the Capabilities\n\nregister is set to 1 and this function is active when the DMA\n\nEnable in the Transfer Mode register is set to 1.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hostsdmabuffersize {
    #[doc = "0: 4KB(Detects A11 Carry out)"]
    H0 = 0,
    #[doc = "1: 8KB(Detects A12 Carry out)"]
    H1 = 1,
    #[doc = "2: 16KB(Detects A13 Carry out)"]
    H2 = 2,
    #[doc = "3: 32KB(Detects A14 Carry out)"]
    H3 = 3,
    #[doc = "4: 64KB(Detects A15 Carry out)"]
    H4 = 4,
    #[doc = "5: 128KB(Detects A16 Carry out)"]
    H5 = 5,
    #[doc = "6: 256KB(Detects A17 Carry out)"]
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
#[doc = "Field `HOSTSDMABUFFERSIZE` reader - To perform long DMA transfer, System Address register shall be\n\nupdated at every system boundary during DMA transfer. These\n\nbits specify the size of contiguous buffer in the system memory.\n\nThe DMA transfer shall wait at the every boundary specified by\n\nthese fields and the HC generates the DMA Interrupt to request\n\nthe HD to update the System Address register.\n\nThese bits shall support when the DMA Support in the Capabilities\n\nregister is set to 1 and this function is active when the DMA\n\nEnable in the Transfer Mode register is set to 1."]
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
    #[doc = "4KB(Detects A11 Carry out)"]
    #[inline(always)]
    pub fn is_h0(&self) -> bool {
        *self == Hostsdmabuffersize::H0
    }
    #[doc = "8KB(Detects A12 Carry out)"]
    #[inline(always)]
    pub fn is_h1(&self) -> bool {
        *self == Hostsdmabuffersize::H1
    }
    #[doc = "16KB(Detects A13 Carry out)"]
    #[inline(always)]
    pub fn is_h2(&self) -> bool {
        *self == Hostsdmabuffersize::H2
    }
    #[doc = "32KB(Detects A14 Carry out)"]
    #[inline(always)]
    pub fn is_h3(&self) -> bool {
        *self == Hostsdmabuffersize::H3
    }
    #[doc = "64KB(Detects A15 Carry out)"]
    #[inline(always)]
    pub fn is_h4(&self) -> bool {
        *self == Hostsdmabuffersize::H4
    }
    #[doc = "128KB(Detects A16 Carry out)"]
    #[inline(always)]
    pub fn is_h5(&self) -> bool {
        *self == Hostsdmabuffersize::H5
    }
    #[doc = "256KB(Detects A17 Carry out)"]
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
#[doc = "Field `HOSTSDMABUFFERSIZE` writer - To perform long DMA transfer, System Address register shall be\n\nupdated at every system boundary during DMA transfer. These\n\nbits specify the size of contiguous buffer in the system memory.\n\nThe DMA transfer shall wait at the every boundary specified by\n\nthese fields and the HC generates the DMA Interrupt to request\n\nthe HD to update the System Address register.\n\nThese bits shall support when the DMA Support in the Capabilities\n\nregister is set to 1 and this function is active when the DMA\n\nEnable in the Transfer Mode register is set to 1."]
pub type HostsdmabuffersizeW<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, Hostsdmabuffersize>;
impl<'a, REG> HostsdmabuffersizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "4KB(Detects A11 Carry out)"]
    #[inline(always)]
    pub fn h0(self) -> &'a mut crate::W<REG> {
        self.variant(Hostsdmabuffersize::H0)
    }
    #[doc = "8KB(Detects A12 Carry out)"]
    #[inline(always)]
    pub fn h1(self) -> &'a mut crate::W<REG> {
        self.variant(Hostsdmabuffersize::H1)
    }
    #[doc = "16KB(Detects A13 Carry out)"]
    #[inline(always)]
    pub fn h2(self) -> &'a mut crate::W<REG> {
        self.variant(Hostsdmabuffersize::H2)
    }
    #[doc = "32KB(Detects A14 Carry out)"]
    #[inline(always)]
    pub fn h3(self) -> &'a mut crate::W<REG> {
        self.variant(Hostsdmabuffersize::H3)
    }
    #[doc = "64KB(Detects A15 Carry out)"]
    #[inline(always)]
    pub fn h4(self) -> &'a mut crate::W<REG> {
        self.variant(Hostsdmabuffersize::H4)
    }
    #[doc = "128KB(Detects A16 Carry out)"]
    #[inline(always)]
    pub fn h5(self) -> &'a mut crate::W<REG> {
        self.variant(Hostsdmabuffersize::H5)
    }
    #[doc = "256KB(Detects A17 Carry out)"]
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
    #[doc = "Bits 0:11 - This register specifies the block size for block data transfers for\n\nCMD17, CMD18, CMD24, CMD25, and CMD53. It can be accessed\n\nonly if no transaction is executing (i.e after a transaction has\n\nstopped). Read operations during transfer return an invalid value\n\nand write operations shall be ignored.\n\n12'h0000: No Data Transfer\n\n12'h0001: 1 Byte\n\n12'h0002: 2 Bytes\n\n12'h0003: 3 Bytes\n\n12'h0004: 4 Bytes\n\n........\n\n12'h01FF: 511 Bytes\n\n12'h0200: 512 Bytes\n\n........\n\n12'h0800: 2048 Bytes"]
    #[inline(always)]
    pub fn transferblocksize(&self) -> TransferblocksizeR {
        TransferblocksizeR::new(self.bits & 0x0fff)
    }
    #[doc = "Bits 12:14 - To perform long DMA transfer, System Address register shall be\n\nupdated at every system boundary during DMA transfer. These\n\nbits specify the size of contiguous buffer in the system memory.\n\nThe DMA transfer shall wait at the every boundary specified by\n\nthese fields and the HC generates the DMA Interrupt to request\n\nthe HD to update the System Address register.\n\nThese bits shall support when the DMA Support in the Capabilities\n\nregister is set to 1 and this function is active when the DMA\n\nEnable in the Transfer Mode register is set to 1."]
    #[inline(always)]
    pub fn hostsdmabuffersize(&self) -> HostsdmabuffersizeR {
        HostsdmabuffersizeR::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - This register specifies the block size for block data transfers for\n\nCMD17, CMD18, CMD24, CMD25, and CMD53. It can be accessed\n\nonly if no transaction is executing (i.e after a transaction has\n\nstopped). Read operations during transfer return an invalid value\n\nand write operations shall be ignored.\n\n12'h0000: No Data Transfer\n\n12'h0001: 1 Byte\n\n12'h0002: 2 Bytes\n\n12'h0003: 3 Bytes\n\n12'h0004: 4 Bytes\n\n........\n\n12'h01FF: 511 Bytes\n\n12'h0200: 512 Bytes\n\n........\n\n12'h0800: 2048 Bytes"]
    #[inline(always)]
    #[must_use]
    pub fn transferblocksize(&mut self) -> TransferblocksizeW<BlksizSpec> {
        TransferblocksizeW::new(self, 0)
    }
    #[doc = "Bits 12:14 - To perform long DMA transfer, System Address register shall be\n\nupdated at every system boundary during DMA transfer. These\n\nbits specify the size of contiguous buffer in the system memory.\n\nThe DMA transfer shall wait at the every boundary specified by\n\nthese fields and the HC generates the DMA Interrupt to request\n\nthe HD to update the System Address register.\n\nThese bits shall support when the DMA Support in the Capabilities\n\nregister is set to 1 and this function is active when the DMA\n\nEnable in the Transfer Mode register is set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn hostsdmabuffersize(&mut self) -> HostsdmabuffersizeW<BlksizSpec> {
        HostsdmabuffersizeW::new(self, 12)
    }
}
#[doc = "Block size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blksiz::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blksiz::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlksizSpec;
impl crate::RegisterSpec for BlksizSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`blksiz::R`](R) reader structure"]
impl crate::Readable for BlksizSpec {}
#[doc = "`write(|w| ..)` method takes [`blksiz::W`](W) writer structure"]
impl crate::Writable for BlksizSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets BLKSIZ to value 0"]
impl crate::Resettable for BlksizSpec {
    const RESET_VALUE: u16 = 0;
}
