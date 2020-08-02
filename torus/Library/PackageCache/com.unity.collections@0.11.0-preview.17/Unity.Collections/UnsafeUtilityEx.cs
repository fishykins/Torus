using System;
using Unity.Mathematics;

namespace Unity.Collections.LowLevel.Unsafe
{
    /// <summary>
    /// Unsafe utility extensions.
    /// </summary>
    public unsafe static class UnsafeUtilityExtensions
    {
        /// <summary>
        /// Swaps content of two buffers.
        /// </summary>
        /// <param name="destination">Destination memory pointer.</param>
        /// <param name="source">Source memory pointer.</param>
        /// <param name="size">Size.</param>
        /// <exception cref="System.InvalidOperationException">Thrown if source and destination memory regions overlap.</exception>
        internal static void MemSwap(void* destination, void* source, long size)
        {
            byte* dst = (byte*)destination;
            byte* src = (byte*)source;

#if ENABLE_UNITY_COLLECTIONS_CHECKS
            if (dst + size > src && src + size > dst)
            {
                throw new InvalidOperationException("MemSwap memory blocks are overlapped.");
            }
#endif

            var tmp = stackalloc byte[1024];

            while (size > 0)
            {
                var numBytes = math.min(size, 1024);
                UnsafeUtility.MemCpy(tmp, dst, numBytes);
                UnsafeUtility.MemCpy(dst, src, numBytes);
                UnsafeUtility.MemCpy(src, tmp, numBytes);

                size -= numBytes;
                src += numBytes;
                dst += numBytes;
            }
        }

        /// <summary>
        /// Reads an element to an unsafe buffer after bounds checking.
        /// </summary>
        /// <typeparam name="T">Type of data in the array.</typeparam>
        /// <param name="source">Source memory pointer.</param>
        /// <param name="index">Index into array.</param>
        /// <param name="capacity">Array capacity, used for bounds checking.</param>
        /// <returns>Element read from the array.</returns>
        /// <exception cref="System.IndexOutOfRangeException">Thrown if reading outside of the array's range.</exception>
        /// <remarks>Reading data out of bounds from an unsafe buffer can lead to crashes and data corruption.
        /// <seealso cref="UnsafeUtility.ReadArrayElement{T}(void*, int)"/> does not do any bounds checking, so it's fast, but provides no debugging or safety capabilities.
        /// This function provides basic bounds checking for <seealso cref="UnsafeUtility.ReadArrayElement{T}(void*, int)"/> and should be used when debuggability is required over performance.</remarks>
        public unsafe static T ReadArrayElementBoundsChecked<T>(void* source, int index, int capacity)
        {
            if ((index > capacity - 1) || (index < 0))
            {
                throw new IndexOutOfRangeException($"Attempt to read from array index {index}, which is out of bounds. Array capacity is {capacity}. This may lead to a crash or reading invalid data.");
            }

            return UnsafeUtility.ReadArrayElement<T>(source, index);
        }

        /// <summary>
        /// Writes an element to an unsafe buffer after bounds checking.
        /// </summary>
        /// <typeparam name="T">Type of data in the array.</typeparam>
        /// <param name="destination">Destination memory pointer.</param>
        /// <param name="index">Index into array.</param>
        /// <param name="value">Value to write into array.</param>
        /// <param name="capacity">Array capacity, used for bounds checking.</param>
        /// <exception cref="System.IndexOutOfRangeException">Thrown if element would be written outside of the array's range.</exception>
        /// <remarks>Writing data out of bounds to an unsafe buffer can lead to crashes and data corruption.
        /// <seealso cref="UnsafeUtility.WriteArrayElement{T}(void*, int, T)"/> does not do any bounds checking, so it's fast, but provides no debugging or safety capabilities.
        /// This function provides basic bounds checking for <seealso cref="UnsafeUtility.WriteArrayElement{T}(void*, int, T)"/> and should be used when debuggability is required over performance.</remarks>
        public unsafe static void WriteArrayElementBoundsChecked<T>(void* destination, int index, T value, int capacity)
        {
            if((index > capacity - 1) || (index < 0))
            {
                throw new IndexOutOfRangeException($"Attempt to write to array index {index}, which is out of bounds. Array capacity is {capacity}. This may lead to a crash or data corruption.");
            }

            UnsafeUtility.WriteArrayElement<T>(destination, index, value);
        }
    }
}
